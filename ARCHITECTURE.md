# Mimarlık

** ÇALIŞMA DEVAM EDIYOR **

Bu belge Fyrox'un üst düzey mimarisini ve temel kavramlarını açıklamaktadır.
Motor mimarisinin temellerini öğrenin ve modifikasyonlarınız için doğru yeri bulun.

## Genel Bakış

Fyrox, çok az sayıda değiştirilebilir parçaya sahip yekpare bir oyun motorudur. Bu, Fyrox'un kendisinin nispeten
modüller arasında güçlü bağlantı. Bununla birlikte, bazı parçaları bağımsız kasalar olarak kullanılabilir - çekirdek, kullanıcı arayüzü ve
ses motordan bağımsızdır. Dahili kaplin çoğu yerde tek yönlüdür, bu da şu anlama gelir
Örneğin, bir renderer bir sahneye **bağlıdır**, ancak sahne renderer hakkında **hiçbir şey bilmez**.
Bu gerçek, motordaki değişiklikleri yeni başlayanlar için bile çok kolay hale getirir.

Fyrox dört kasadan oluşur - fyrox-core, fyrox-sound, fyrox-ui ve Fyrox'un kendisi. fyrox-core, fyrox-sound ve
fyrox-ui **standalone** kasalardır ve ayrı ayrı kullanılabilirler, bu üçünün buluştuğu tek yer
Önceden her sandığın ayrı bir deposu vardı, ancak daha sonra her şeyi tek bir depoya koymaya karar verdim
Çünkü motoru kullanan herhangi bir projeyi inşa etmek çok zahmetliydi.

Bir başka önemli gerçek de Fyrox'un ECS kullanmadığı, bunun yerine nesilsel arenalar (Fyrox'un
terminoloji) verimli bellek yönetimi için (hızlı tahsis/ayırma, CPU önbellek verimliliği). Bu şu anlama gelir
bitişik bellek bloğuna (havuz) yerleştirilmiş eski güzel yapılarla çalıştığınızı unutmayın. Bir kez
nesnesi bir havuza yerleştirildiğinde, nesneye erişmek (ödünç almak) için kullanılabilecek bir tanıtıcı elde edersiniz
ihtiyaç duyduğunuzda. Bu yaklaşım, nesneler arasında herhangi bir ilişki kurmanıza olanak tanır - tutamaç sadece bir çift
sayıları, borç denetleyicisinde sorunlara neden olmaz. Daha fazla bilgi için kontrol edin
[pool.rs](https://github.com/FyroxEngine/Fyrox/blob/master/fyrox-core/src/pool.rs).

### Çekirdek

Çekirdek, motorun diğer bölümlerinde kullanılan bazı çok yaygın algoritmaları ve veri yapılarını içerir.
Doğrusal cebir, hızlandırıcı yapılar, renk uzayı fonksiyonları vb. içerir. Başka bir deyişle şunları içerir
Motorun diğer bölümlerinde çok yaygın olarak kullanılan "yapı taşları".

### Renderer

Fyrox, ertelenmiş + ileri işleyicilerin bir kombinasyonunu kullanır. Ertelenmiş işleyici opak nesneleri işlemek için kullanılır,
saydam nesneleri işlemek için forward renderer kullanıldığında. Renderer, çok yaygın olan birçok
grafiksel efektler. Renderer ihtiyaçların çoğu için uygundur, ancak henüz yeterince esnek değildir ve
henüz özel gölgelendiriciler kullanmanın bir yolu yok.

### Kullanıcı Arayüzü

Fyrox özel kullanıcı arayüzü kütüphanesi kullanır. Düğüm tabanlıdır, çok güçlü bir düzen sistemine sahiptir, mesajlar kullanır
widget'lar arasında iletişim kurmak için, şekillendirmeyi destekler. Kütüphanede 30'dan fazla widget vardır (yerleştirme yöneticisi dahil,
pencereler, dosya tarayıcıları vb.) Lütfen kütüphanenin hiçbir şey oluşturmadığını, bunun yerine
sadece her türlü renderer ile kullanılabilecek bir dizi çizim komutu hazırlar - bir yazılım (GDI için
örneği) veya bir donanım (OpenGL, DirectX, Vulkan, Metal, vb.).

### Ses

Fyrox yazılım ses motoru [fyrox-sound](https://github.com/FyroxEngine/Fyrox/tree/master/fyrox-sound) kullanır.
Ses motoru, HRTF kullanarak binaural ses oluşturma desteği sağlar, bu da mükemmel ses verir
mekansallaştırma.

## Kod Haritası

Kod haritası, değişiklikleriniz için doğru yeri bulmanıza yardımcı olacaktır. Bu, kod haritasının en sıkıcı kısmıdır.
belgesinin içindekiler bölümünü rahatınız için burada bulabilirsiniz:

- [fyrox-core](#fyrox-core)
  - [math](#mathmodrs)
- [fyrox-ui](#fyrox-ui)
  - [widgets](#borderrs)
- [fyrox-sound](#fyrox-sound)
  - [buffer](#buffermodrs)
  - [decoder](#decodermodrs)
  - [device](#devicemodrs)
- [Fyrox](#fyrox)

### fyrox-core

Yukarıda da belirtildiği gibi, fyrox-core sadece bir dizi faydalı algoritmadan ibarettir. Eğer bir şey eklemek istiyorsanız
bağımlı kasalarda kullanılacaksa doğru yerdesiniz. İşte çok kısa açıklama
her modülün.

#### math/mod.rs

Modül, bazı kesişim kontrol algoritmaları, vektör projeksiyon yöntemleri (üç düzlemli haritalama için
örneğin), genel dikdörtgen yapısı ve kolayca sınıflandırılamayan diğer şeyler.

#### math/aabb.rs

Modül, Eksen Hizalı Sınırlayıcı Kutu (AABB) yapısını içerir. Hızlandırmak için sınırlayıcı bir hacim olarak kullanılır
uzamsal kontroller (ışın dökümü, kaba kesişim kontrolleri vb. gibi).

#### math/frustum.rs

Modül Frustum yapısını içerir. Amacı (projede) görünürlük kontrollerini gerçekleştirmektir - için
örnek kamera, hangi nesnelerin ekranda işlenmesi gerektiğini belirlemek için frustum culling kullanıyor.

#### math/plane.rs

Modül, temelde sadece düzlem denklemini temsil eden Düzlem yapısını içerir. Düzlemler şunlar için kullanılır
intersection tests.

#### math/ray.rs

Modül, kesişim testleri için kullanılan Ray yapısını içerir. Örneğin motor ışınları kullanır
gölgeleri hesaplamak için ışın izleme yapmak için lightmapper'da.

#### math/triangulator.rs

Modül, çokgen üçgenleme için bir dizi üçgenleme algoritması içerir. İki algoritma vardır:
basit olanı dörtgenler içindir ve genel olanı kulak kırpma algoritmasıdır. Modüldeki şeyler
GPU'da işlenmeye uygun hale getirmek için 3B modellerdeki çokgenleri üçgenleştirmek için kullanılır.

#### color.rs

Modül, HSV ve RGB renk uzaylarında renklerle çalışmak ve renkleri dönüştürmek için yapı ve yöntemler içerir.
HSV <-> RGB renkleri.

#### color_gradient.rs

Modül, çok noktalı basit doğrusal renk gradyanı içerir. Parçacık sistemlerinde yaygın olarak kullanılır
zaman içinde parçacıkların rengini değiştirmek için. Örneğin bir kıvılcım beyaz renkten başlar ve zamanla daha kırmızı hale gelir.
zaman ve sonunda siyah olur.

#### lib.rs

Modül BiDirHashMap ve çok az sayıda başka algoritma içerir.

#### numeric_range.rs

Modül hatasız sayısal aralık içerir - yanlış sınırlara sahip bir aralık oluşturmanın yolu yoktur - bounds
örnekleme anında belirlenecektir.

#### octree.rs

Modül, ışın dökümünü, nokta problamayı hızlandırmak için kullanılan Octree hızlandırma yapısını içerir,
kutu kesişim testleri vb.

#### pool.rs

Modül, motorun kalbini içerir: havuz, motorda en sık kullanılan yapılardan biridir.
Amacı, bitişik bir bellek bloğunda bir türdeki nesneler için bir depolama alanı sağlamaktır. Herhangi bir nesne
daha sonra bir tanıtıcı tarafından erişilebilir. Tanıtıcılar bir çeşit indekstir, ancak ek bilgilerle birlikte
tutamacın geçerli olup olmadığını kontrol etmek için kullanılır.

#### profiles.rs

Modül basit bir müdahaleci profilleyici içerir. Bir kapsamı zamanlamak için özel makro (scope_profile!()) kullanır.

#### rectpack.rs

Modül, dikdörtgen paketleme algoritması (en yaygın olarak "bin-packing problemi" olarak bilinir) içerir. Bu kullanılır
doku atlasları oluşturmak için.

#### visitor.rs

Modül, düğüm tabanlı serileştirici/serileştirici (ziyaretçi) içerir. Motorda serileştirilen her şey
bu serileştirici. Temel tiplerin serileştirilmesini, birçok std tipini (aşağıdakiler dahil) destekler
Rc/Arc) ve kullanıcı tanımlı tipler.

### fyrox-ui

fyrox-ui bağımsız, grafik API'sinden bağımsız, düğüm tabanlı, genel amaçlı bir kullanıcı arayüzü kütüphanesidir.

#### lib.rs

Modül, Kullanıcı Arayüzü yapısını ve Kontrol özelliğini içerir.

#### border.rs

Modül, temelde değişken kenarlık genişliğine sahip bir dikdörtgen olan Border widget'ını ve
alt widget'lar için bir kapsayıcı olma yeteneği.

#### brush.rs

Modül, grafik ilkellerini çizmenin bir yolunu tanımlayan Brush yapısını içerir.

#### button.rs

Modül, Button widget'ını ve oluşturucusunu içerir.

#### canvas.rs

Modül Canvas widget'ını ve oluşturucusunu içerir. Canvas, alt widget'lar için basit bir kapsayıcıdır,
alt widget'ların konumunu doğrudan ayarlamaya izin verir.

#### check_box.rs

Modül, CheckBox widget'ını ve oluşturucusunu içerir. CheckBox üç durumludur (işaretli, işaretli değil,
tanımlanmamış) anahtar.

#### color.rs

Modül, renkler ve oluşturucuları ile çalışmak için widget'lar içerir. Değiştirmek için ayrı widget'lar vardır
ton, doygunluk, parlaklık ve bileşik renk seçici widget'ı.

#### decorator.rs

Modül, decorator widget'ını ve oluşturucusunu içerir. decorator, alt widget'lar için basit bir kapsayıcıdır,
En yaygın eylemler için yerleşik davranışa sahiptir: fare sınırlara girdiğinde veya çıktığında görünümü değiştirir,
kullanıcı üzerine tıkladığında, vb.

#### dock.rs

Modül, Yerleştirme Yöneticisi ve Tile widget'ları ile bunların oluşturucularını içerir. Yerleştirme yöneticisi şunları birleştirebilir
Karolarında birden fazla Pencere widget'ı, her karo yeniden boyutlandırılabilir, yerleştirilebilir veya geri alınabilir. Bu sahip olunması gereken
her türlü editör için widget.

#### draw.rs

Modül "çizimden" sorumludur. Tırnak içinde, çünkü aslında hiçbir şey çizmez, sadece
çizim komutlarını daha sonra yorumlanmak üzere bir kuyruğa gönderir.

#### dropdown_list.rs

Modül, DropdownList widget'ını ve oluşturucusunu içerir. DropdownList seçilenleri gösteren küçük bir alandır
öğesi ve öğelerin listesini içeren bir "açılır pencere".

#### expander.rs

Modül, Expander widget'ını ve oluşturucusunu içerir. Genişletici, alt widget'lar için daraltılabilir bir kapsayıcıdır
bir içeriği tanımlayan bir alan ile.

#### file_browser.rs

Modül, dosya sistemini görüntüleyen bir dizi widget içerir. FileBrowser widget'ı basit bir dosya sistemidir
ağacı, FileSelector, FileBrowser ve birkaç düğme içeren bir penceredir.

#### formatted_text.rs

Modül, metin biçimlendirme ve "işleme "den sorumludur. İkincisi tırnak içinde, çünkü kütüphane
sadece bir yazı tipindeki glif bilgisini kullanarak her bir glifi bir metin satırında kelime kaydırma ve diğer
kullanışlı özellikler (metin boyutu hesaplama vb. gibi).

#### grid.rs

Modül, Grid widget'ını ve oluşturucusunu içerir. Grid güçlü bir düzen aracıdır, alt öğelerin düzenlenmesine izin verir
widget'larını bir dizi satır ve sütunda bir araya getirir.

#### image.rs

Modül, Image widget'ını ve oluşturucusunu içerir. Görüntü, dokusu olan basit bir dikdörtgendir.

#### list_view.rs

Modül, ListView widget'ını ve oluşturucusunu içerir. ListView düzenlenmiş öğeler için bir kapsayıcıdır
bir yığın olarak. ListView öğe seçimini destekler.

#### menu.rs

Modül menü widget'ları içerir. Buradaki menü, kök öğeleri olan bir şerit olan klasik bir menü anlamına gelir ve bir
alt menüler kümesi.

#### message.rs

Modül, kütüphanedeki her widget için desteklenen tüm mesajları içerir.

#### messagebox.rs

#### node.rs

#### numeric.rs

#### popup.rs

#### progress_bar.rs

#### scroll_bar.rs

#### scroll_panel.rs

#### scroll_viewer.rs

#### stack_panel.rs

#### tab_control.rs

#### text.rs

#### tree.rs

#### ttf.rs

#### utils.rs

#### vec.rs

#### vector_image.rs

#### widget.rs

#### window.rs

#### wrap_panel.rs

### fyrox-sound

fyrox-sound, birden fazla işleyiciye ve yüksek kaliteli sese sahip bağımsız bir ses motorudur. Ses motoru
HRTF kullanarak binaural ses oluşturma desteği sağlar ve bu da mükemmel ses uzamsallaştırması sağlar.

#### buffer/mod.rs

#### buffer/generic.rs

#### buffer/streaming.rs

#### decoder/mod.rs

#### decoder/vorbis.rs

#### decoder/wav.rs

#### device/mod.rs

#### device/alsa.rs

#### device/coreaudio.rs

#### device/dsound.rs

#### device/dummy.rs

### Fyrox

Motorun kendisi. Şunlara sahiptir: bir renderer, kaynak yöneticisi, animasyonlar, sahneler ve çeşitli yardımcı programlar
lightmapper, uv-mapper, navigasyon ağı, logger, pathfinder.

#### animation/mod.rs

#### animation/machine/mod.rs

#### animation/machine/blend_nodes.rs

#### engine/mod.rs

#### engine/error.rs

#### engine/resource_manager.rs

#### renderer/mod.rs

#### renderer/framework/mod.rs

#### renderer/framework/framebuffer.rs

#### renderer/framework/geometry_buffer.rs
