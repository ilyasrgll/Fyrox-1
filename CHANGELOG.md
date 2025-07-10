# 0.36.1

Bazı can sıkıcı hatalar için düzeltmeler içeren küçük sürüm.

## Düzeltildi

- Kafesler için frustum culling sorunları düzeltildi.
- Devralınabilir özellikleri canlandırırken değiştirilmiş olarak doğru şekilde işaretleyin.
- Seçim metni temizlenirken varlık tarayıcısında seçili öğe gösterildi.
- Düğüm kaldırma iletişim kutusunun hatalı genişliği düzeltildi.
- Fizik senkronizasyonu sorunu düzeltildi.
- Yerleştirme yöneticisi düzeninin kaydedilmesi ve yüklenmesi düzeltildi.
- Proje adı doğrulaması iyileştirildi (hem `fyrox-template` hem de proje yöneticisinde).
- Belgelerdeki örnek gölgelendirici düzeltildi.
- Editördeki yüzen paneller için isimler eklendi - yüklemeden sonra yanlış düzen düzeltildi.

# 0.36

Bu sürüm alt sandıkların sürümlerini birleştiriyor - artık tüm alt sandıklar aynı sürüme sahip. Bu, taşımayı kolaylaştırır
gelecek sürümlere.

## Eklendi

- Döşeme haritaları.
- UI stil desteği.
- Aynı anda birden fazla Fyrox projesini yönetmek için proje yöneticisi.
- Açılır liste dokümanları.
- Sprite sayfası animasyon varlıkları için PartialEq uygulandı.
- Yüzey Veri Kaynağı için özellik düzenleyici.
- Yüzey kaynağı için yüzey veri görüntüleyicisi.
- `BaseSceneGraph::remove_nodes`.
- Etkileşim modlarını dinamik olarak ekleme/kaldırma yeteneği.
- Çarpıştırıcılar için şekil düzenleme.
- Sprite tabanlı aygıtlar için gölgelendirici (sprite tabanlı aygıtları diğer her şeyin üzerine çizmeye izin verir).
- `math::get_arbitrary_line_perpendicular`.
- ScrollBar`daki değer göstergesi için yazı tipi + boyutunu belirleme yeteneği eklendi.
- Bir Button oluştururken yazı tipi boyutunu belirleme yeteneği.
- Pencere başlığı oluştururken yazı tipi ve yazı tipi boyutu belirleme özelliği eklendi.
- Yüzey kaynak yükleyicisi eklendi.
- Yerleşik yüzeyler.
- `Executor` için yapılandırılabilir bir gaz kelebeği kare aralığı eklendi.
- Düzenleyicinin büyük fırçalar tarafından aşırı yüklenmesini önlemek için fırça işlemleri için akıllılık kontrolü eklendi.
- grid widget'ı için mesajlar
- satırları / sütunları değiştirme / kenarlık / kenarlık kalınlığı çizme yeteneği.
- `Malzeme::doku` yardımcı yöntemi.
- `Color::repeat_opaque` yöntemi.
- `DrawingContext::push_grid` yöntemi.
- Kaynak için `save + save_back` yöntemleri.
- Varlık tarayıcısı için "yenile" düğmesi eklendi.
- `ResourceDataRef::as_loaded_ref/mut` yöntemleri.
- Varlıkları çift tıklama kullanarak açabilme.
- `ListVIew` widget'ı için çoklu seçim desteği.
- `impl PartialEq for Ray`.
- Sahne gizmo'sunu kullanarak düzenleyici kamerasını döndürme yeteneği ekleyin.
- `ImmutableString` için `impl From<&String>`.
- Geliştirilmiş materyal api - `Material::set_property` artık çok daha az ayrıntılı.
- 3DS max'ten fbx malzemeleri için daha iyi destek.
- 2d çarpıştırıcılar için doğrulama.
- Varlık tarayıcısına klasörler eklendi.
- Arazide delik açma yeteneği.
- Işık kaynakları için deneysel oklüzyon ayıklama.
- Ham baytlar yerine yazılan pikselleri almak için `read_pixels_of_type`.
- `R32UI` doku formatı eklendi.
- gpu dokusu için `get_image`.
- Async çerçeve arabelleği okumaları için piksel arabelleği.
- Önbellek boyutlarını render istatistiklerine dahil edin (kontrol edilemeyen GPU bellek kullanım artışını yakalamaya yardımcı olur).
- Varlık tarayıcısında kaynakları çoğaltma yeteneği.
- Parçacık sistemleri için görünür mesafe eklendi.
  - Performansı artırmak için uzaktaki parçacık sistemlerini otomatik olarak işleme dışında bırakır.
  - Sistem bazında ince ayar yapılabilir.
- Özel gölgelendiricilerden makas testini etkinleştirebilme/devre dışı bırakabilme.
- Özel gölgelendiricilerde derinlik işlevini belirleyebilme.
- Tek tip tamponlar eklendi.
- Birden fazla UBO'nun daha kolay işlenmesi için `UniformBufferCache` eklendi.
- Bağlama grupları + render kaynakları aracılığıyla zorunlu doku bağlama eklendi.
- Grafik sunucusu yeteneklerini getirme yeteneği.
- Deneysel `UniformMemoryAllocator`.
- Işık kaynakları için frustum culling.
- Editör penceresinin büyütülmüş bayrağını kaydetme / geri yükleme desteği.
- Tüm açık sahneleri bir kerede + kısayol tuşlarıyla kaydedebilme.
- `AxisAlignedBoundingBox::project` yöntemi.
- `Plugin` için `post_update` geri çağrısı.
- Editör eklentileri konteyneri
- eklenti araması için bazı yararlı yöntemler ekler.
- Daha fazla yerleştirilebilir pencere.
- Eğri düzenleyici widget'ında seçimi kopyalama/yapıştırma yeteneği.
- Aşırı şişmeyi önlemek için mesaj günlüğü için yapılandırılabilir bir sınır eklendi.
- Parçacık sistemleri için yapılandırılabilir koordinat sistemi - oluşturulan parçacıklar için bir koordinat sistemi seçmeye izin verir-
  yerel veya dünya.
- Parçacık sistemleri için aydınlatma desteği.
- `ModelResource::instantiate_and_attach` yöntem.
- Aynı anda birden fazla eğriye anahtar ekleyebilme.
- Eğri düzenleyici widget'ı için `sığdırmak için yakınlaştır` kısayol tuşu.
- Erken dönüş ifadeleri için faydalı makrolar. let-else mevcut olsa da, hala olması gerekenden daha fazla kod satırı alır.
  Bu makrolar çok daha kompakt ve okunması daha kolay.
- `BaseControl::self_size` yöntemi.
- Editör kullanıcı arayüzü istatistik eklentisi. Düzenleyici tarafından kullanılan toplam widget miktarının izlenmesine olanak tanır; bu, aşağıdakilerin olup olmadığını bulmak için yararlıdır "sarkan" widget'lar var.
- `DockingManagerLayoutDescriptor::has_window` yöntemi.
- Geçerli çerçeve için ui'nin toplam çizim komutu sayısını yazdırır.
- `Pencere` widget'ı için `remove_on_close` bayrağı.
- Bir widget'ın alt widget'ları için özel sıralama uygulayabilme.
- Menü öğelerini sıralayabilme.
- Editörde işlenmiş kullanıcı arayüzü mesajlarını izleme - mesaj kuyruğunun aşırı yüklenmesini bulmaya yardımcı olur.
- `has_component` yardımcı yöntemleri.
- `StyleResource` kaynak türü.
- UI mesajları için yapılandırılabilir yönlendirme stratejisi.
- Pencere simgesini daha kolay ayarlamak için yardımcı yöntemler.
- Düzenleyici ayarlarına bir `Zed` düzenleyici seçeneği eklendi.
- Araç ipuçları için yapılandırılabilir gecikme eklendi.
  - Araç ipuçlarının farenin üzerine gelindiğinde anında açılmasını önler, bunun yerine yapılandırılabilir bir süre vardır (varsayılan olarak 0,55 sn)
    Gecikme.
  - Fareyi hareket ettirirken çıkan rahatsız edici araç ipucunu kaldırır.
- Daha fazla doku ayarı eklendi - temel seviye, maksimum seviye, min lod, maksimum lod, lod bias.
- Dosya tarayıcı widget'ı için ev/masaüstü dizinleri kısayol düğmeleri eklendi.
- Dosya tarayıcı araçlarında geçerli yola odaklanabilme.
- Grafik sunucusu yapıcısını belirleyebilme.
  - Esasen grafik sunucularını oluşturma / çalışma zamanı aşamalarında değiştirme yeteneği verir.
  - Varsayılan olarak hala OpenGL grafik sunucusunu kullanır.
- Yazı tipleri için karakter aralığı desteği eklendi.
- `BuildContext::send_message` yöntemi eklendi.
- Proje yöneticisi CI eklendi.
- `Untyped->Typed` kaynağının deserializasyonu için geriye dönük uyumluluk.
- Eleman tamponu için kullanım belirleme yeteneği.
- `info! + warn! + err!` günlük makroları.
- Dokümantasyon iyileştirmeleri.
- Kod şişkinliğini gidermek için `Downcast` özelliği.
- Malzeme düzenleyicideki gölgelendirici alanı için araç ipucu eklendi.
- Geçiş düğmesi widget'ı.
- Yansıma için etiketler eklendi.
- `WidgetBuilder::with_uniform_margin(..)`.
- Düzenleyici ayarlarındaki gruplar için kısayollar: belirli bir ayar grubuna hızlıca atlamayı sağlar.
- Düzenleyici ayarları için arama işlevi.
- `Rect<T>` için `impl TypeUuidProvider.`
- <Option<Rect<T>> için özellik düzenleyicileri eklendi.
- Dokuz yama widget iyileştirmesi.
  - Atlas desteği için bir doku bölgesi belirleme yeteneği eklendi.
  - Açık uv koordinatlarını kaldırın ve bunları anında hesaplayın.
  - Dokuz yama widget'ının merkez bölgesinin çizimini devre dışı bırakma yeteneği.
  - Dokuz yama widget'ı için yapılandırılabilir döşeme modu.
  - Yeni doku dilimi düzenleyicisini kullanarak doku diliminin daha kolay düzenlenmesi.
- Sürüklenebilir şeyler için başparmak widget'ı.
- ScrollViewer aracının dikey ve yatay kaydırmasını değiştirmek için mesajlar.

## Değişti

- Her kaynak dosyaya proje lisansı eklendi.
- Sahne düğümü dönüşümünü kök yaparken kimliğe sıfırlayın.
- Widget'ları bağlarken z indeksini dikkate alın.
- Fyrox-template'i lib + cli'ye bölün.
- Şablon çekirdeği için proje kök direktörünü belirleme yeteneği.
- İsteğe bağlı uygulama argümanları. Program argümanlarını ayrıştırmaya çalışırken çökmeyi önler.
- Daha sezgisel yukarı/aşağı hareketi yapmak için tuş bağlamalarını değiştirin.
- `SurfaceSharedData`, `Resource<SurfaceData>` olarak değiştirildi.
  - Yüzey paylaşılan verileri zaten bir tür kaynaktı.
  - Kafeslerin harici olarak kaynak olarak kaydedilmesine izin verir.
  - Yüzey verileri için standart kaynak işlem hattının kullanılmasına izin verir.
- Düzenleyicide basitleştirilmiş kamera seçme API'si.
- Geliştirilmiş arazi fırça sistemi.
- Özellik düzenleyicide yüzey kaynağı türünü yazdırın.
- Yeni nesne yerleşimi düzeltildi.
  - Çocuk nesneler (0,0,0) konumunda kalacaktır.
  - "Oluştur" menüsü aracılığıyla oluştururken, yeni bir nesne kameranın önüne yerleştirilecek.
  - Üstü kök olan bir üst nesne oluştururken, o da kameranın önünde yer alacaktır.
- Denetçi widget'ının isim sütunu genişliğini belirleyebilme.
- Kamera projeksiyon modunu editör ayarlarına kaydedin.
- Yeniden düzenlenen düzenleyici kamera denetleyicisi - 2d modunda mmb kullanarak kamerayı sürüklemeye izin verir.
- Yerleşik kaynakların öğelerini sıralayın.
- Şekli oluşturulamadığında yerel çarpıştırıcıyı kaldırın.
- ABSM'deki animasyon konteynerinden animasyonlar üzerindeki kontrolü kaçırın - artık ABSM kullandığı animasyonları kendisi güncelliyor,
  ve yalnızca o anda bir durum veya aktif geçiş durumları tarafından kullanılanlar.
- Render çerçevesini ayrı bir sandığa çıkarın.
- Ağ geometrisinin fbx öğelerini isteğe bağlı hale getirin.
  - Bir uyarı mesajı yazdırır ve okumaya devam eder.
  - Bu, yalnızca animasyon fbx'leri gibi örgü geometrisi olmayan "hatalı biçimlendirilmiş" fbx'leri yükleyebilmek için gereklidir.
- Yürütücüde varsayılan olarak kaynak sıcak yeniden yüklemeyi etkinleştirin.
- `RotateVec2`yi paylaşılan gölgelendirici işlevlerine taşıyın.
- Yerleşik kaynakların ilk verilerini ve dosya uzantısını (varsa) saklayın.
- Opengl başlatma işlemi render çerçevesine taşındı.
- Doku matrisi depolaması yerine kemik matrisleri için tek tip tampon kullanın.
- Nesne örneği verilerini gölgelendiricilere aktarmak için tek tip tampon kullanın.
- Kamera özellikleri kendi tekdüze bloğuna taşındı.
- Renderer genelinde tek tip tamponlara geçildi.
- Malzeme özelliklerini tek tip tamponlar kullanarak geçirin.
  - Malzeme özellikleri için otomatik olarak tek tip tampon açıklaması oluşturun.
  - Örnekleyiciler için üniformaları otomatik olarak tanımlayın.
  - Artık gölgelendiricilerde malzeme özelliklerini manuel olarak tanımlamaya gerek yok, sadece `properies.your_property_name` kullanın.
- Gpu programının opengl'e özgü kodu kendi modülüne izole edildi.
- Gpu'ya tek tip veri yüklemesini hızlandırmak için tek tip bellek ayırıcı kullanın.
  - Render paketlerinin işlenmesini iki adıma böler: tek tip veri toplama + yükleme ve gerçek render.
  - Tekdüze tamponlarda mevcut tüm alanı kullanarak belleğin daha verimli kullanılması (tekdüze olmasını önler.
    Gpu'daki gerçek bellek bloğunun 4 kb olduğu yerlerde sadece 200-300 bayt bellek içeren tamponlar).
  - Genel olarak bireysel veri aktarımlarının ve gapi çağrılarının sayısını önemli ölçüde azaltır.
  - Performansı %12-15 oranında artırır.
- Gereksiz tampon bağlama/bağlama kaldırma - api çağrılarında biraz zaman kazandırır (özellikle WebGL'de her şeyin
  JS aracılığıyla proxied).
- sceneDepth dokusunu gölgelendiricilere açıkça aktarın.
- Dokular için açık bağlama kullanın. Bir şeyler çizerken düzinelerce `glUniform1i` çağrısını önler, böylece iyileştirir
  performansı %5-10 (her gl çağrısının JS'den geçirildiği WebAssembly'de daha fazla).
- Kaynak bağlarını içerecek şekilde gölgelendirici yapısı yeniden düzenlendi.
  - Gölgelendirici yapısını daha katı hale getirir ve örtük yerleşik değişkenleri kaldırır.
  - Kaynakların bağlanma noktalarını açık hale getirir.
- `Matris2Editor` genel boyutta `MatrixEditor` haline getirildi.
- Gölgelendirici özellik adında değişmez dize kullanın.
- Malzemeler yeniden düzenlendi.
  - Materyal artık yalnızca değiştirilen gölgelendirici özelliklerini saklıyor.
  - Doğrulamayı set_property/bind'dan renderer'a taşıyın, burada sadece günlüğe bir hata mesajı yazdırır
    eğer bir şeyler yanlışsa.
  - Doku kaynağı bağlayıcısından geri dönüş değeri kaldırıldı, bu bilgiyi çoğaltmanın bir anlamı yok çünkü doğru
    biri yine de gölgelendiricide saklanır.
  - Gölgelendiricilerdeki doku tanımından `default` özelliği kaldırıldı.
- Bir render paketi oluştururken ışık bilgisi toplanır. Sahne grafiği düğümleri üzerindeki gereksiz döngü kaldırılır.
- Dylib tabanlı dışında özel dinamik eklentilere izin vermek için sıcak yeniden yükleme refactor.
- Geliştirilmiş gpu doku api.
- Çökmeleri önlemek için düğüm mesajı işlemede kontrollü ödünç alma gerçekleştirin. Bir düğüm zaten silinmişse çökme meydana gelebilir,
  ancak mesajı hala kuyruktaydı.
- Düğümlerden bileşen sorgulama `ComponentProvider` özelliği ile değiştirildi.
- Editör denetçisi bir eklentiye dönüştürüldü.
- Düzenleyiciden Sahne kaydedilirken Sahne ayarlarını korumak için Grafiği klonlarken fiziği klonlama.
- TabControl iyileştirmeleri...
- Bir çift handle ve ref döndürmek için `traverse_iter` değiştirildi - bir
  Aynı anda bir referansla işleyin, zaten çift iş olan yeniden ödünç almaya gerek yok.
- Animasyon izleri verilerini paylaşılan bir kaynağa ayıran `AnimationResource` eklendi.
  - Animasyonları klonlarken bellek tüketimini önemli ölçüde azaltır, çünkü izleri klonlaması gerekmez
    Artık.
  - Animasyon kaynağı, aynı parçaları kullanan birden fazla animasyon arasında paylaşılabilir.
  - Animasyon oynatıcı sahne düğümünün örneklenmesini önemli ölçüde hızlandırır.
  - Geriye dönük uyumluluk korunur.
- Araç çubuğunun kendisine odaklanırken arama çubuğunun metin kutusuna odaklanın - araç çubuğu odağı zaten bir anlam ifade etmiyor, çünkü
  klavye ile etkileşime girer, ancak metin kutusu etkileşime girer.
- Düğüm seçici kullanılabilirlik iyileştirmeleri.
  - Açıkken arama çubuğuna odaklanma.
  - Enter tuşu ile seçimi onaylayabilme.
  - Açıldığında ilk seçilen öğeyi görünüme getirme.
  - Sekme gezintisi eklendi.
- İsteğe bağlı yerine tembel z-indeks sıralaması.
- Örnek tamponunu canlandırılabilir özellikler listesinden hariç tutma.
- Geliştirilmiş özellik seçici.
  - Açılışta arama çubuğuna odaklanın.
  - Sekme gezintisi.
  - Yeniden bağlama sırasında seçilen özellikleri vurgulayın.
  - Enter tuşuna basarak seçimi onaylayabilme özelliği.
- Düzenleyicinin malzeme ile ilgili kısımları kendi eklentisine ayrıldı - malzeme düzenleyici artık varsayılan olarak mevcut değil ve
  sadece ihtiyaç duyulduğunda oluşturulur, bu da bellekten (hem ram hem de vram) ve cpu/gpu zamanından tasarruf sağlar.
- Ragdoll sihirbazını ayrı bir eklentiye ayırdı.
- Ayarlar penceresini ayrı bir eklentiye taşıyın.
- Animasyon düzenleyicisini kendi eklentisine taşıyın.
- Geliştirilmiş editör eklentileri api.
- Animasyon düzenleyicisi daha önce yerleştirilmişse düzenleyici başlangıcında animasyon düzenleyicisi oluşturun.
- Absm düzenleyiciyi ayrı bir eklentiye taşıyın.
- İsteğe bağlı prefabrikler için dosya kaydetme seçicisi oluşturun.
- Eğri düzenleyici penceresini kendi eklentisine taşıyın.
- Yol sabitleyiciyi bir eklentiye taşıyın.
- Düzenleyicide oluşturulan kafesler için yerleşik yüzeyler kullanın.
- En son `tinyaudio`ya taşındı.
- Sabit kodlanmış kullanıcı arayüzü widget kurucuları kaldırıldı.` ConstructorProvider` özelliği aracılığıyla kullanıcı tanımlı kurucularla değiştirildi.
- Oluşturma menülerinde menü öğelerini alfabetik sıraya göre sıralayın.
- Kodlanmış kullanıcı arayüzü stil değişkenleri yapılandırılabilir stillerle değiştirildi.
- Vuruş testi için araç ipuçlarını görünmez yapın.
- Günlük panelini `fyrox-ui`ye taşıyın.
- Etkin açılır pencere tamamen gösterilinceye kadar düzenleyiciyi çalışır durumda tutun.
- Dosya tarayıcısının varsayılan yolunu `./` olarak değiştirin.
- Günlük dosyasını varsayılan olarak devre dışı bırakın. Günlük dosyası bazı durumlarda istenmeyebilir ve şimdi varsayılan olarak kapalıdır ve
  `fn main` içinde `Log::set_file_name/set_file` tarafından etkinleştirildi.
- Günlük dosyasını değiştirmek için açık api.
- Editörde tescilli Arial yazı tipi Roboto ile değiştirildi.
- Motor başlatıldığında yerleşik gölgelendiricileri önceden derlemeyin.
  - Bunları talep üzerine derlemek daha hızlıdır.
  - WebAssembly üzerinde böyle bir derleme 10-15 saniye sürebilir.
- Doku ile ilgili kodu ayrı bir sandığa ayırdı. Dokuları doğrudan kullanmak için `fyrox-ui`ye eklenmesine izin verir.
  hacky `UntypedResource` kullanarak.
- `TextureResource`u mümkün olduğunca doğrudan kullanıcı arayüzü kodunda kullanın - yazılmamış↔yazılmış dönüşümlerle gereksiz hokkabazlığı ortadan kaldırır.
- `Image` widget'ını ölçüm aşamasında doku boyutunu kullanmaya zorlayın - daraltılmış görüntü ile "şaşırtıcı etkiyi" ortadan kaldırır, eğer
  genişlik/yükseklik açıkça ayarlanmamıştır.
- Ses başlatma hataları artık ölümcül değil. Motorun uygun ses çıkışı olmayan ortamlarda çalıştırılmasına izin verir
  destek.
- Düzenleyici sürümünü pencere başlığına yazdırın.
- Düzenleyici sürümünü başlangıçta günlüğe yazdırın.
- Motorun sabit kodlanmış sürümünü Cargo.toml'daki ile değiştirin. Bu yarı güvenilir bir çözümdür, ancak çok
  kodlanmış versiyona sahip olmaktan daha iyidir.
- Seçim sırasında projeksiyon (2d/3d) seçiciyi kapatın.
- Dünya görüntüleyicide "parça seçimi" için geçiş düğmesini kullanın.
- Dünya görüntüleyicisinin arama çubuğunu diğer düğmelerle aynı satıra yerleştirin.
- `Load_image` `fyrox-ui` utils'e taşındı.

## Sabit

- Bulanık yazı tipleri düzeltildi.
- Editör performansı önemli ölçüde iyileştirildi.
- En son Rapier fiziğine geçişten sonra eklem stabilitesi iyileştirildi.
- İlgili mesajdaki z indeksini kullanın.
- İlgili mesajı kullanarak pencere başlığını değiştirmeye çalışırken oluşan çökme düzeltildi.
- Prosedürel kafes serileştirme düzeltildi.
- Seçili nesneyi başka bir türle değiştirirken denetçi senkronizasyonu düzeltildi.
- Fyrox-math'de Rect testleri düzeltildi.
- `transmute_vec_as_bytes` sağlamlık düzeltmesi.
- Doku alanına doku olmayan bir nesneyi sürükleyip bırakmaya çalışırken oluşan çökme düzeltildi.
- Varlık silme işleminden sonra varlık tarayıcısını yenileyin.
- Çarpıştırıcılar için daha iyi doğrulama.
- FBX'te zincirleme doku düğümleri için destek - en son 3ds max/Maya/vb'de yapılan FBX dosyalarında normal harita içe aktarımını düzeltir.
- Geçerli dizindeki değişiklikleri izleyin ve varlık tarayıcısı içeriğini yenileyin.
- UI düğümleri klonlanırken olası çökme düzeltildi.
- Proje dışa aktarıcıda araç yükleme kontrolü düzeltildi.
  - Önceden yüklenmiş araçları yüklemeye çalışmayın.
  - Gerçek bir ihtiyaç olmadığında ağa erişmeyi önler.
- Boru hattına zaten bağlıysa gereksiz doku bağlama düzeltildi.
- Sınırlayıcı şekle geçirmeden önce rotasyon matrisinden ölçeklendirmeyi atın - ışık kaynaklarının kırpma sorunlarını düzeltir.
- Hiçbir parça yanmasa bile ışık saçılımı oluşturmayı atlamayın. Işık saçılımının yanıp sönmesini düzeltir.
- Gölge haritası lod seçim koşulu düzeltildi.
- Animasyon eğrisi verilerine erişimi hızlandırın.
- Daha küçük hale getirmek için `ValueBinding` içinde `ImmutableString` kullanın, daha hızlı kopyalama ile sonuçlanır (32 bayta karşı 16 bayt).
- Render hedeflerinin doku önbelleğine birden fazla kez kaydedilmesi engellendi.
- Render veri toplama performansı iyileştirildi.
- Proje dışa aktarıcı alt süreçleri için devralınan `RUSTFLAGS` bırakıldı.
- Büyük paketler işlenirken oluşan çökme düzeltildi.
- Veri için zaten yeterli alan varsa gpu tamponunu yeniden tahsis etmeyin.
- Veri boş olduğunda tampon yazma komutlarını yoksay.
- Glsl es hassasiyetini `highp` olarak ayarlayın.
- Hidpi ekranında ikinci başlangıçta geçersiz editör penceresi boyutu düzeltildi.
- Vektör görüntülerinin ayarlanmış bir boyuta sahip olduğundan emin olun.
- MacOS'ta yol ilk kez ayarlandığında bildirim sandığında çökme düzeltildi.
- Geri dönüş dokularını kendi yapılarına izole ederek kod şişkinliği azaltıldı.
- Kullanımdan kaldırılan PanicInfo kullanımı nedeniyle başarısız olan wasm testleri düzeltildi.
- Işık kaynağı sınırlayıcı kutusu hesaplanırken ölçeklendirme kısmı çıkarıldı.
- Bazı canlandırılamayan özellikler özellik seçiciden hariç tutuldu.
- Hiyerarşik özelliklerin yayılma mükemmelliği grafik boyutundan çıkarıldı.
  - Grafik artık hiyerarşik özellikleri yalnızca gerçekten değişenler için güncelliyor.
  - Statik sahnelerde performansı önemli ölçüde artırır.
- 2d rijit gövdeler için gereksiz global dönüşüm güncellemesi önlendi.
- "Işınlanma" hatası düzeltildi (bir sahne düğümü bir kare boyunca dünyanın orijininde bulunduğunda ve sonra geri ışınlandığında
  olması gereken yerde).
- `Vector_to_quat`da potansiyel nan önlendi.
- Yankı ses efektindeki yakınsama düzeltildi.
- Döngü animasyonlarında kök hareketi titreşimi düzeltildi - döngü sınırları yanlış işleniyordu, bu nedenle hataya yol açıyordu
  Bazı yinelemelerden sonra can sıkıcı titreşime yol açan birikim.
- Nokta ışıklarının etrafındaki görünür sınırlar düzeltildi.
- Motorun iç kısımlarındaki kod şişkinliği azaltıldı.
- Çarpıştırıcıların dönüşüm senkronizasyonu düzeltildi.
- `Inspector` widget senkronizasyon sorunları düzeltildi.
- Birden fazla animasyon parçasını aynı anda silerken oluşan çökme düzeltildi.
- Izgara ve Metin dahil olmak üzere kullanıcı arayüzü düzeni için düzeltme.
- Silinen bir çarpıştırıcıdan kesişimleri almaya çalışırken oluşan çökme düzeltildi.
- Kök durum olmadan animasyon olaylarını toplamaya çalışırken oluşan çökme düzeltildi.
- Bazı kafeslerde `accurate_world_bounding_box` kullanıldığında oluşan çökme düzeltildi - bir kafesin pozisyonu/kemiği yoksa çöküyordu
  tepe tamponundaki indeksler/kemik ağırlıkları nitelikleri.
- Ragdoll sihirbazı tarafından oluşturulan ragdoll ekleminin adı düzeltildi.
- Genel editör performansı ve özellikle ui düğümleri bağlantısı iyileştirildi.
- Düzenleyici ayarları penceresinin gereksiz senkronizasyonunu önleyin - ~%10 zaman tasarrufu sağlar.
- Düzenleyicinin aynı dokuyu tekrar tekrar yüklemesini önleyin.
- Ağaç kökü için klavye gezintisi düzeltildi - klavye odağının ağaç kökünde kalmasına neden olan can sıkıcı sorun giderildi.
- Kamera önizleme paneli boyutu düzeltildi.
- Bazı widget'ların silinmesi düzeltildi.
- Öğelerini dinamik olarak değiştirirken menü öğesinin ok görünürlüğü düzeltildi.
- `MenuItem` performans sorunları düzeltildi.
- Malzeme düzenleyici gölgelendirici alanının senkronizasyonu düzeltildi.
- Esasen editörden derleme araçları içeren `fyrox-build-tools` sandığı eklendi.
- Yanlış doku bağlamalarının geçersiz kılınması düzeltildi - bazı nesnelere uygulanan yanlış dokularla ilgili garip bir hataya neden oldu (çok
  pencereyi yeniden boyutlandırdıktan sonra kullanıcı arayüzünde fark edilir).
- Düzenleyicideki simgeleri yumuşatmak için düzenleyicideki simgeler için mip eşleme kullanın.
- Kenarlık' widget'ı oluşturma sırasında arka plan renginin "sızması" düzeltildi.
- Hacim dokuları için R koordinatlarının senkronizasyon hatası düzeltildi.
- Görsel dönüşüm hesaplamasındaki dönüşüm sırası düzeltildi.
- `BinaryBlob` serileştirilirken yanlış bellek hizalaması düzeltildi.
- Doku olmadan dokuz yama widget'ı kullanıldığında oluşan çökme düzeltildi.
- Doku alanına doku dışı kaynak bırakıldığında oluşan çökme düzeltildi.

## Kaldırıldı

- Dokulardaki gereksiz veri karması hesaplaması kaldırıldı.
- Render veri paketinden gereksiz alan kaldırıldı - `is_skinned` bayrağı anlamsız, çünkü türetilebilir
  Zaten kemik matrisi sayımından ve her zaman demet başına değil, örnek başına tanımlanır.
- Gereksiz dekal katman indeksini mesh/terrain/render veri demetinden kaldırın. Bunlar öncesinden kalan artıklardır
  özel malzeme dönemi, çıkartma katmanı indeksi malzemelerde tanımlandığından ve bu alanlar sadece
  Etkisi yok.
- Derinlik ofseti kaldırıldı.
  - Gölgelendiriciler ile yapılabilirdi.
  - Her render edilen örnek için gereksiz projeksiyon matrisi hokkabazlığı eklediği için kaldırıldı.
- Malzeme gölgelendiricilerine aktarılan örtük karışım şekilleri depolaması kaldırıldı - artık doğrudan `Mesh` düğümünden kontrol ediliyor,
  ve karışım şekli deposunu açıkça geçmek için geçici malzeme oluşturur.
- `PersistentIdentifier` ve `MatrixStorageCache` kaldırıldı.
- `BaseLight`dan `cast_shadows` özelliği kaldırıldı - bu özellik bir noktada gereksiz olmaya başladı, çünkü `Base`
  zaten böyle bir özelliğe sahiptir ve `BaseLight` içindeki özellik karışıklığı önlemek için silinmelidir.
- Animasyon düzenleyicisindeki yanlış hata mesajı kaldırıldı.
- `Node::query_component_ref/mut` kaldırıldı.
  - Mevcut işlevselliği çoğaltıyor.
  - `SceneGraphNode::component_ref/mut` ile değiştirildi.
- Animasyon değerleri uygulanırken gereksiz kutulama kaldırıldı - keyfi sayısal özelliklerin animasyonunu önemli ölçüde yapar
  Daha hızlı.

# 0.35

- Alt krates sürüm birleştirmesi nedeniyle sürüm atlandı. Daha fazla bilgi için 0.36 değişiklik günlüğüne bakın.

# 0.34.1 Motor + 0.21.1 Editör

- Editörde kök için ebeveyn oluşturmaya çalışırken oluşan çökme düzeltildi
- Her şey başlatıldıktan ve güncellendikten sonra komut dosyası mesajlarını gönderme
- Seçim türü adının denetçide kaybolmasını önleme
- Varlık örneklemesi geri alınırken oluşabilecek çökme düzeltildi
- Düzenleyicide 2d projeksiyon modunda render sorunları düzeltildi
- Render dönüşümü değiştiğinde bir widget'ın görsel dönüşümünü güncelleme

# 0.34 Motor + 0.21.0 Editör

## Eklendi

- Eklentiler için kod sıcak yeniden yükleme.
- Bir sahne düğümünde birden fazla komut dosyasına sahip olma yeteneği.
- Kafesler için statik ve dinamik gruplama.
- Otomatik dağıtım için proje dışa aktarıcı.
- Editör için yapılandırılabilir derleme profilleri.
- Birden fazla kullanıcı arayüzü örneğine sahip olma yeteneği.
- GLTF desteği (`gltf` özelliği ile kullanılabilir).
- Kullanıcı arayüzünde klavye navigasyon desteği.
- Varlık tarayıcısındaki varlıklar için önizleme oluşturma.
- Sahne önizlemesi için ızgara.
- Kodun çalışırken yeniden yüklenmesini destekleyen projeler oluşturmak için `fyrox-template` geliştirmeleri.
- `AnimationPlayer` + `AnimationBlendingStateMachine` widget`ları.
- Onları örnekleme yeteneğine sahip UI prefabları.
- `Pool::try_get_component_of_type` + `MultiBorrowContext` için aynı.
- NodeTrait::on_unlink` yöntemi.
- `Node` için `ComponentProvider` özelliği uygulandı.
- `MultiBorrowContext::get/get_mut` yöntemleri.
- Nesneleri çoklu ödünç bağlamından kaldırma yeteneği.
- `newtype_reflect` temsilci makrosu.
- `SceneGraph::change_hierarchy_root` yöntemi.
- UI sahne kökünü değiştirebilme.
- UI widget'ları için özellik kalıtımı.
- Dünya görüntüleyicisine/sahne önizleyicisine prefab bırakarak UI prefablarını örnekleyebilme.
- Editörün denetçisinden komut dosyalarını açabilme.
- `Control::post_draw` yöntemi.
- Bir sahne düğümünün çocuklarını yeniden sıralama yeteneği.
- `SceneGraph::relative_position` + `SceneGraphNode::child_position` yöntemleri.
- Dünya görüntüleyicisinde düğümleri/araçları yeniden sıralayabilme.
- Widget'lar için daha fazla simge eklendi.
- Animasyon düzenleyicisinde kullanıcı arayüzü animasyonları için destek eklendi.
- Yapılandırılabilir UI güncelleme anahtarları.
- Absm editöründe ui absm düğümlerini düzenleme yeteneği.
- AbsmEventProvider' widget'ı.
- Grafik bağlamını başlatırken msaa'yı etkinleştirebilme.
- `Border` widget'ında köşe yarıçapını değiştirebilme.
- UI çizim bağlamında köşeleri yuvarlatılmış dikdörtgenler çizebilme.
- Bulanıklığı önemli ölçüde azaltan `fyrox-ui` için düzen yuvarlama eklendi.
- FBX'te gömülü dokular için destek eklendi.
- `Selector` widget.
- Editöre cli args olarak açmak için proje dir ve sahneler eklendi.
- `utils::make_cross_primitive` yardımcı yöntemi.
- UI çizim bağlamında tel daire çizme yeteneği.
- VectorImage widget'ında WireCircle ilkellerini çizebilme.
- Daha fazla test.
- Vertex buffer API iyileştirmeleri.
- Editör için render istatistikleri penceresi.
- Fizikte şekil dökümü eklendi.
- Malzeme düzenleyicide dokuların atamasını kaldırma yeteneği.
- Animasyon düzenleyicide animasyonlar için negatif oynatma hızı ayarlamaya izin verin.
- Daha kolay sahne klonlama için `Scene::clone_one_to_one` kısayolu.
- Motoru dinamik olarak bağlayabilmek için `fyrox-dylib` crate.
- Motoru dinamik olarak editöre bağlayabilme.
- Tiplenmemiş dokular için özellik editörü eklendi.
- `Eklenti::on_loaded` yöntemi eklendi.
- `NetListener::local_address` yöntemi eklendi.
- `Model::new` yöntemi.
- Serileştirme sırasında `InheritableVariable` alan optimizasyonunu devre dışı bırakabilme.
- Desteklenen tüm platformlar için proje şablonu için CI eklendi.
- Tanjantlar hesaplanırken dejenere üçgenler için tanılama eklendi.
- `Havuz::first_ref/first_mut` yöntemleri.
- Android proje şablonları için sürüm anahtar deposu eklendi.
- Sahne başına işleme istatistiklerini toplayın.
- `transmute_slice` yardımcı işlevi.
- GPU doku verilerini okuma yeteneği.
- HDR için deneysel histogram tabanlı otomatik pozlama (varsayılan olarak devre dışı).
- `curve` için kısa yol açı enterpolasyon modu - `Curve::angle_at`.
- `RcUiNodeHandle` türü için özellik düzenleyici.
- Uyarlanabilir kaydırma çubuğu başparmak.
- Kaynak yöneticisinden mevcut görev havuzunu getirme yeteneği.
- Varlık tarayıcısındaki varlıklar için asenkron simge oluşturma.
- Büyük/küçük harfe duyarsız dize karşılaştırma yardımcı yöntemi `fyrox::core::cmp_strings_case_insensitive`.
- Varlık tarayıcısında arama için büyük performans iyileştirmesi.
- Animasyonlar için yapılandırılabilir interpolasyon modu.
- `Esc` tuşunu kullanarak açılır pencereleri kapatabilme.
- Bir pencerenin boş adı varsa uyaran yerleştirme yöneticisi düzeni için tanılama eklendi.
- Ağaç widget'ı için klavye navigasyonu.
- `Esc` tuşu ile pencereleri kapatabilme.
- Açılan pencereye otomatik olarak odaklanma.
- Menü aracı için klavye navigasyonu.
- `ImmutableString` editörü eklendi.
- Denetçi modülü için dokümanlar.
- `Esc` tuşunu kullanarak menüleri devre dışı bırakabilme.
- Mesajları bir açılır pencereden bir `widget`a yeniden göndermek için `PopupMessage::RelayedMessage`.
- `NavigationLayer` widget'ı `Tab`/`Shift+Tab` navigasyonunu işler.
- Boşluk tuşunu kullanarak onay kutusu durumunu değiştirebilme.
- `Boşluk`/`Enter` tuşlarını kullanarak düğme widget`ına tıklayabilme.
- Klavye etkileşimi için kullanılabilecek widget'lar için `accepts_input`.
- Denetçideki giriş alanları için klavye gezintisi eklendi.
- Klavye odağı olan bir widget'ı vurgulayın.
- Ziyaretçi dokümanları.
- Ok tuşlarını kullanarak açılır listeyi açma/kapama yeteneği.
- Enum özellik düzenleyicisinde `Variant` mesajını yeniden yayınlayın.
- Açılışta açılır pencere içeriğine (varsa) odaklanma.
- Liste görünümü widget'ı için klavye navigasyonu.
- Açıldığında pencere içeriğine (varsa) odaklanma.
- Odaklanılan öğeyi gezinme katmanında görünüme getirmek için isteğe bağlı yetenek.
- Oyunu editörden çalıştırmak için kısayol tuşu (varsayılan `F5`).
- Ok tuşları ile `NumericUpDown` widget değerini artırma/azaltma yeteneği.
- Yapılandırılabilir komut yığını maksimum kapasitesi (komut yığınının kontrolsüz bir şekilde büyümesini önler, bu da çok fazla
  editör uzun süre çalışıyorsa bellek).
- Odaklanan `TextBox` widget'ında metni otomatik seçme.
- Sahneyi manuel olarak render edebilme.
- `VecEditor` widget'ı için hassasiyet ayarlayabilme.
- Sahne önizlemesinde gölgeli ve tel kafes modu arasında geçiş yapabilme.
- Eğri düzenleyici aracı için çoklu eğri desteği.
- Önceden tanımlanmış renklere sahip `Color::COLORS` dizisi.
- Eğri düzenleyicideki her eğri için farklı fırçalar ayarlayabilme.
- Animasyon düzenleyicisindeki eğrilere farklı renkler uygulayabilme.
- Animasyon düzenleyicisinde parça seçerken aynı anda birden fazla eğri gösterme.
- Açılır menü widget'ı.
- Izgara tutturma için hızlı erişim menüsü.
- Sahne düğümleri için `create parent` bağlam menüsü seçeneği.
- Eğri düzenleyici widget'ına arka plan eğrileri konsepti ekleyin.
- Yeni oluşturulan nesneler için akıllı yerleştirme.
- Mesh kontrol paneli eklendi - birkaç tıklamayla fizik varlıkları (çarpıştırıcılar, katı cisimler, vb.) oluşturmaya izin verir.
- Bir türün montaj adını almak için `Reflect::assembly_name`.

## Değiştirildi

- Editör kullanıcı arayüzü için büyük stil iyileştirmeleri.
- Rapier 0.18'e geçiş yapıldı.
- Çoklu ödünç alma bağlamı yeniden düzenlendi - statik boyut kısıtlaması kaldırıldı ve ödünç alma takibi dinamik ve daha fazlası yapıldı
  verimli.
- Daha iyi UX için çoklu borçlanma için `Option` yerine `Result` kullanın.
- Sarkan havuz kayıtlarını önlemek için `Ticket::drop` üzerine panik eklendi.
- Genel grafik işleme kodu `fyrox-graph` crate içine taşındı.
- Her widget için `Control::update` çağrısı yapmayın:
  - editörde karmaşık sahnelerde ortalama performansı %13-20 oranında artırır.
  - Eğer `Control::update`in çağrılmasını istiyorsanız, widget'ı oluştururken `need_update` bayrağını ayarlamanız gerekir.
- `Control::update` içinde kullanıcı arayüzüne değiştirilebilir erişim.
- Dinamik gönderim kullanmak için `Selection` yeniden düzenlendi.
- Tüm editör komut sistemi dinamik gönderim kullanacak şekilde yeniden düzenlendi.
- `SceneGraph` özelliğini nesne güvenli ve nesne güvenli olmayan parçalara ayırın.
- Grafik bağlamı olmasa bile `Engine::pre_update` mantığının çoğunu çalıştırın.
- Performansı artırmak için renk uzayı dönüşümü parçacık sisteminin tepe gölgelendiricisine taşındı.
- Bir ağın dünya uzayı sınırlayıcı kutusunu `update` yerine `sync_transform` üzerinde yeniden hesaplayın.
- Rectpacker, `Pool` yerine düz `Vec` kullanacak şekilde yeniden düzenlendi.
- Dikdörtgenle ilgili kod `rectutils` sandığına taşındı.
- Tamam bir kaynak kaydediliyorsa hatalı kaynakların kaydını otomatik olarak kaldırın.
- Uvgen'in gerçek yüzey verilerini değiştirmesini önleyin.
- uvgen modülü `uvgen` sandığına çıkarıldı.
- Octree'de havuz yerine basit vec kullanın.
- `math` + `curve` + `octree` modları `fyrox-math` sandığına taşındı.
- Işık eşleyici `lightmap` sandığına taşındı.
- Navmesh ajanı için geriye doğru hareket (negatif hız) desteği.
- Motor uygulaması `fyrox-impl` sandığına taşındı, `fyrox` sandığı artık bunun bir proxy'si.
- Etkileşim modları paneli araç çubuğuna taşındı.
- Manuel olarak oluşturabilmek için gölgelendirici yöntemleri herkese açık hale getirildi.
- Dikkat çekmek için atanmamış tutamaçları turuncu renkte gösterin.
- `TextBox` widget'ının büyük ölçüde yeniden düzenlenmesi, çalışmayı çok daha keyifli hale getiriyor.
- DockingManager` kutucukları için önemli kullanılabilirlik iyileştirmeleri.
- `window` widget içeriği artık `NavigationLayer` widget örneği ile bağlantılıdır.
- `TextBox` ve `NumericUpDown` widget'larının değişmedikleri halde değişiklik mesajı göndermeleri engellendi.
- Geçerli seçimin dünya uzayı konumu için genişlik ve hassasiyet azaltıldı.
- Yinelenen dizelerde bellek tüketimini azaltmak için sahne düğümleri ve widget'lar için `ImmutableString` kullanın.
- Çeşitli grafik sorunlarını önlemek için sahneleri değiştirirken renderer'ı flush etmeyin.
- Animasyon düzenleyicisindeki eğriler için daha bilgilendirici isimler.
- Eğri düzenleyicide tuşları seçerken/sürüklerken imleç simgesini değiştirin.
- Eğri düzenleyicideki koordinat sisteminin büyük ölçüde yeniden düzenlenmesi.
- Animasyon düzenleyicisinde animasyon oynatıcısını seçili tutun.
- Kamera montajının düz nesnelerle çalışmasını sağlamak için AABB geçerliliği sıfır boyutlu boyutları içerecek şekilde değiştirildi.
- Sahnede düğüm seçerken prefabrik kökleri tercih edin.
- Plugin özelliği için `Reflect` özelliği bağlandı.

## Sabit

- Kademeli gölge haritaları (CSM) oluşturma düzeltildi.
- Parçacık ortaya çıkma oranı çok yüksek ayarlandığında oluşan çökme düzeltildi.
- MultiBorrowContext kullanılırken UB düzeltildi.
- Klonlanmış widget'ın görünürlüğü düzeltildi.
- Widget kopyaları için benzersiz kimlik ayarlayın.
- Sahneleri kapatırken oluşan çökme düzeltildi.
- `pool` için `Default` impl düzeltildi.
- Bir şey yazarken `TextBox` widget'ındaki nadir çökme düzeltildi
- Arazi iç kısımlarında çift piksel döngüsü (y üzerinde iki kez döngü yapıyordu) düzeltildi.
- Düzenleyicide bir MenuItem oluşturma düzeltildi.
- UI widget'ı animasyonlu ise düzeni yeniden hesaplamaya zorlayın
- Tüm UI özellikleri için kayıtlı özellik düzenleyicileri.
- Hatalı FBX küme yüklemesi düzeltildi (FBX modellerinin hatalı işlenmesini düzeltir)
- Seçim aralığı `TextBox` widget'ında yanlış olduğunda oluşan çökme düzeltildi.
- Silinen sahne düğümüne referans veren bir parçayı yeniden bağlamaya çalışırken animasyon düzenleyicisinde oluşan çökme düzeltildi.
- Dosya tarayıcı aracında yol oluştururken ağaç öğelerini düzgün şekilde genişletin.
- Dosya tarayıcı aracında disklerin altındaki öğelerin iki katına çıkarılması düzeltildi.
- Animasyon düzenleyicisinde parça silme düzeltildi.
- Boş dosya yolunda dosya tarayıcı davranışı düzeltildi
- Varlık tarayıcısında geçerli dir'i seçin.
- Bağlantısı kesilen dinleyicileri günlükten otomatik olarak kaldırın.
- `ListView` widget'ının özel düzen paneli desteği düzeltildi.
- WebAssembly hedefindeki asenkron görevler düzeltildi.
- İç değişebilirliğe sahip türler için özellik kalıtımı düzeltildi.
- Fareyi bir `Decorator` widget`ının üzerine getirdiğinizde seçili fırçayı koruyun.
- `TabControl` widget başlıklarının stili düzeltildi.
- SearchBar widget stili geliştirildi.
- Hatalı komut dosyası görev işleme düzeltildi (görev sonucunu tüm komut dosyalarına aktarıyordu, bunun yerine
  görev).
- Ortaya çıkma oranları yüksek olduğunda parçacık sistemlerinin aşırı parçacık ortaya çıkarmasını önleyin.
- Hatalı tepe noktası tampon veri düzeni düzeltildi.
- Varlık çalışırken yeniden yükleme sırasında seçilen bir düğüm silinirse oluşan çökme düzeltildi.
- Varlık tarayıcısında bir klasörün kendi alt klasörüne taşınması engellendi.
- İlgili ışık haritası dokuları silindiğinde ışık haritasının kaydedilmesi düzeltildi.
- Karıştırma sorunlarını önlemek için render alırken dikdörtgenleri arkadan öne doğru sıralayın.
- Şeffaflığa sahip düğümler oluşturulurken arkadan öne sıralama.
- Gökyüzü kutusu küp haritasındaki dikişler düzeltildi.
- `should_be_deleted` alanını gizle.
- Devre dışı bırakılmış düğümlerde komut dosyalarını güncelleme.
- Ses bağlamı serileştirmesini düzeltti (bu hata, yükleme sırasında tüm ses otobüslerinin kaybolmasına neden oluyordu).
- Ses otobüsü düzenleyicisinde olası çökmeyi düzeltti.
- Düzenleyiciyi kapatırken çökmeyi düzeltti.
- Parçacık sistemlerinde `taşma ile çıkarma girişimi` çökmesini düzeltti.
- Yanlış `Selection::is_empty` uygulamasını düzeltti.
- WebAssembly'de render edilen görüntüye sızan tuval arka plan rengi düzeltildi.
- Varlık tarayıcısında arama yaparken `target` dizinini yok say.
- Animasyon düzenleyicide parçaları genişletirken yanlışlıkla etkinleştirme/devre dışı bırakma sorunu düzeltildi.
- Düzenleyici düzeninin kaydedilmesi ve yüklenmesi düzeltildi.
- Genişletici kapatıldığında `Inspector` özelliklerinin kaybolması önlendi.
- Renk gradyanı düzenleyicide düz açılır pencereler yerine bağlam menüleri kullanılır.
- Kaynak oluşturucuda yanlış uzantı önerisi düzeltildi.
- Kaynak oluşturucuda yanlış kaynak oluşturma düzeltildi.
- Bağlama yöneticisinde yavaş döşeme boyutlandırma düzeltildi.
- Etkileşim modlarının sırası aynı tutulur.
- `ScrollPanel` widget'ı için görüntüleme düzeltildi - artık öngörülemeyen şekilde atlamıyor.
- Klavye girişini görünmez widget'lara aktarmayın.
- Eğri sınırlarını hesaplarken kenar durumlarını doğru şekilde işleyin.
- Eğri düzenleyici widget'ındaki “sığdırmak için yakınlaştır” işlevini düzeltin.
- Yeniden boyutlandırma sırasında eğri düzenleyici widget'ındaki görünümün kaymasını düzeltin.
- Frustum culling bayrağı kullanımını düzeltin.
- Denetleyici senkronizasyonunu/bağlam değiştirmeyi düzeltin.
- Boş seçimden seçili varlığı almaya çalışırken meydana gelen çökme düzeltildi.
- Sekmelerdeki `X` düğmesini kullanarak sahneleri kapatırken meydana gelen çökme düzeltildi.

## Kaldırıldı

- `Define_command_stack` makrosu kaldırıldı
- Seçimi değiştir komutundan gereksiz `old_selection` argümanı kaldırıldı

# 0.33.1 Motor + 0.20.1 Editör

## Sabit

- Bir dokuyu derinlemesine klonlarken yaşanan kilitlenme düzeltildi. Editörün arazileri kaydederken takılmasına neden oluyordu (#598).
- Düğüm oluşturma işlemi geri alınırken ara sıra yaşanan çökme düzeltildi
- Klonlanan nesneler için vurgulama düzeltildi

# 0,33 Motor + 0,20 Editör

## Eklendi

- UI editörü.
- Komut dosyaları ve eklentiler için görev sistemi.
- Dinamik yazı tipi atlası uygulandı.
- 2D grafikler için gruplama.
- Varlık Tarayıcısında kaynakları ve klasörleri taşıyabilme.
- Düzenleyicide seçim için kenar vurgulama.
- Varlık tarayıcısından kaynak oluşturma özelliği eklendi.
- `Text` ve `TextBox` widget'ları için yükseklik parametresi eklendi.
- Kullanıcı arayüzü yüklenirken IO soyutlamasını belirleme yeteneği. -` Vec<char>` UTF32 dizelerini düzenlemek için `Utf32StringPropertyEditorDefinition` eklendi.
- `RefCell<T>` türleri için `RefCellPropertyEditorDefinition`.
- Biçimlendirilmiş metin ve örnekleri için yansıma + serileştirmeyi etkinleştirin.
- Yerleşik yazı tipi kaynağı.
- Yazı tipi önizlemeli yazı tipi kaynağı özellik düzenleyicisi.
- Fontları varlık tarayıcısından atayabilme.
- Kaynaklar için yansıma.
- UI grafik manipülasyon yöntemleri.
- `screen` widget'ı otomatik olarak geçerli ekran boyutuna sığar.
- Widget'lar için dünya görüntüleyicide tür adını gösterme.
- `Reflect::apply_recursively` için yok sayılan türleri belirleyebilme.
- Eğri ve hrir kaynakları için önizleme.
- İstenilen konumda pencere açabilme.
- Widget'larda dokuları UntypedResource olarak düzenleyebilme.
- `ErasedHandle` için `Serialize + Deserialize + Display` özellikleri uygulandı.
- Düzenleyicideki bağlamsal yüzen paneller için akıllı konumlandırma.
- `WidgetMessage::Align` + `WindowMessage::OpenAndAlign` mesajları.
- Tüm widget'lar için düzeni bir kerede geçersiz kılma yeteneği.
- Serileştirme sırasında bir struct/enum'un tüm alanlarını isteğe bağlı olarak işaretleyebilme: `#[visit(optional)]` artık
  doğrudan bir struct/enum'a eklenir, böylece alanlardaki diğer tüm bu tür nitelikler geçersiz kılınır.
- Kullanıcı arayüzüne, görev havuzuna, grafik bağlamına, komut dosyaları için geçerli sahne tanıtıcısına erişim eklendi.
- `PluginsRefMut::get/get_mut/of_type_ref/of_type_mut` yöntemleri.
- Hafif performans iyileştirmeleri için `Pool` için bir grup `#[inline]` niteliği eklendi.
- Interrior mutability kullanılarak değiştirilebilen `AtomicHandle` eklendi.
- Piksel türünü `Renderer::render_ui_to_texture` yöntemine geçirme yeteneği.
- Malzeme alanı düzenleyicisinde malzeme kaynağı durumunu gösterme.
- `ScrollViewer` ve `ScrollPanel` widget`ları için içeriğin sonuna kadar kaydırma yeteneği.
- Işık haritalarını bir dosyaya kaydedebilme ve dosyadan yükleyebilme.
- Bir düğme basılı tutulurken tıklamaları tekrarlayabilme.
- Malzemeleri varlık tarayıcısından düzenlemek için açabilme.
- Alan yinelemesi için yansıma kullanırken türlerin bir listesini filtreleyebilme.
- `Handle` türü için `PartialOrd + Ord` özellikleri uygulandı.
- Malzeme kaynakları için varlık tarayıcısına simge eklendi.
- Jenerikleri `uuid_provider` makrosuna geçirme yeteneği.
- Gezinme ağını birden fazla iş parçacığı arasında paylaşabilme.
- `RwLock` için `Reflect` özelliği uygulandı.
- Widget'ları isme göre aramak için `UserInterface::find_by_name_down_from_root` yöntemi.
- UI varlıkları için `Send` özelliği uygulandı.
- Kullanıcı arayüzü kaynağı eklendi.
- Çarpıştırıcıları üst sınırlara sığdırma özelliğine sahip çarpıştırıcı kontrol paneli.
- Vektör görüntüsünün ilkelleri için özellik editörü.
- Abonesi olmayan komut dosyası mesajı olduğunda günlükte uyarı göster.
- Standart tipler için `TypeUuidProvider` özelliği uygulandı.
- `Renderer::render_ui_to_texture`da net renk belirleme yeteneği.
- Gölgelendirici kaynakları için varlık tarayıcısına simge eklendi.
- Widget'ları UI'den UI'ye kopyalayabilme.
- `create` menüsünden ragdoll oluşturma yeteneği.
- Animasyon düzenleyicisinde parçaları yeniden bağlama özelliği eklendi.
- Bir dizi standart malzeme eklendi, bunları düzenleyicide açığa çıkardı.
- Yer tutucu doku eklendi.
- Kaynak içe aktarma seçeneklerini ilgili yükleyicilerinden getirme yeteneği.
- Karakter için `Visit` ve `Reflect` özellikleri uygulandı.
- İlgili önizleme oluşturucularında varlıklar için simgeler belirleyebilme.
- Bir kaynağın doğru varsayılan durumunu ayarlayabilmek için `TypedResourceData` özelliği.
- Yerleşik motor kaynak türleri için `ResourceData::save` özelliği uygulandı.
- LOD'lar için dokümantasyon.
- İsimleri olan renkler için renk sabitleri.
- Kaynakları kaydetme yeteneği.
- `ResourceLoader::supports_extension` yöntemi.
- `VisitError` için `Error` özelliği uygulandı.
- Malzeme::set_texture` kısayolu.
- `ImmutableString` için `From<&str>` özelliği uygulandı.
- Vertex öznitelik tanımlayıcısı için normalleştirme seçeneği eklendi.
- Deneysel ağ soyutlama katmanı eklendi.
- Dikdörtgen düğümü için frustum culling eklendi.
- Kamera görünümü piramit görselleştirme eklendi ([@dasimonde](https://github.com/dasimonde)'a tebrikler).
- Kaynak sistemi için IO soyutlaması eklendi (kudos [@jacobtread](https://github.com/jacobtread)).
- UI widget'ları için `Reflect`, `Debug`, `Visit` özellik uygulamaları eklendi.
- `Useize/isize` için `Visit` özellik uygulaması eklendi.
- ResourceIo::move_file` yöntemi eklendi.
- Filtreleme ile `ResourceManager::move_resource` yöntemi eklendi.
- Düşen yol ile `FileBrowser` için `Drop` mesajı eklendi.
- `ResourceIo::canonicalize_path` eklendi.
- `Pool::generate_free_handles` yöntemleri eklendi.
- Mod için uygun düğme oluşturan `InteractionMode::make_button` yöntemi eklendi.

## Değişti

- Yeni UI sahnelerini desteklemek için büyük editör yeniden düzenlemesi.
- Düşük seviye animasyon modülleri fyrox-animation sandığına taşındı.
  - Sahneye özel animasyon varlıkları + prelüdler için tip takma adları.
  - Sprite sayfası animasyonu için genel parametre olarak doku.
- Yazı tipini kaynağa dönüştürün + `TextMessage::Height` eklendi.
- Standart yerleşik gölgelendiricileri varsayılan olarak prosedürel olmayan hale getirin.
- Kaynakların iç yapısı yeniden düzenlendi.
  - Kaynakla ilgili tüm veriler artık `ResourceHeader` içinde saklanmaktadır.
    `ResourceState`
    varyantlarında ve hatta kaynak verilerinin kendisinde.
  - Geriye dönük uyumluluk korunmuştur.
  - Path+flag yerine `ResourceKind`, yeniden düzenlenmiş kaynak yükleyici özelliği.
- Editördeki etkileşim modları yeniden düzenlendi.
- Kullanıcı arayüzünde SharedTexture'dan UntypedResource'a geçildi
- `ResourceManager::request/try_request` kullanımı basitleştirildi. `rm.request<Type, _>` yazmaya gerek yok, sadece
  `rm.request<Type>`.
- Işık Paneli yüzen panellere kaydedildi, böylece yerleştirilebilir.
- Varlık tarayıcısında arama daha akıllı hale getirildi.
- GPU kaynakları önbelleği yeniden düzenlendi.
- Dokulara erişimi hızlandırın.
- `ScriptTrait::id()` yönteminin otomatik uygulaması. Bu uygulama artık sizin
  Komut dosyaları.
- Düzenleyicide derleme günlüğünün sonuna kaydırın.
- Bir derleme başarısız olduğunda derleme penceresinin kapanması önlendi.
- Düğüm tanıtıcısı özellik düzenleyicisi, kullanıcı arabirimi widget'larıyla da çalışacak şekilde düzenlendi.
- Performansı artırmak için özellik seçicideki doku baytlarını filtreleyin.
- Kaydırma çubuğu artırma/azaltma düğmeleri için tıklama tekrarı modu etkinleştirildi.
- Yakalanan herhangi bir ui öğesi varsa düzenleyiciyi etkin tutun.
- Kaydırma görüntüleyici için kaydırma çubuğu adımı artırıldı.
- `aabb_of_descendants` için filtre argümanı eklendi.
- Animasyon varlıklarında ErasedHandle yerine soyut EntityId kullanın.
- Gezinme ağının iç kısımları optimize edildi.
- Harici kaynakların verilerinin serileştirilmesi önlendi.
- Ekran boyutunu `Control::update`e aktarın.
- Kullanıcı arayüzünü tamamen klonlayabilme.
- Düzenleyicideki sahne kaydetme iletişim kutuları daha kararlı hale getirilmek için yeniden düzenlendi.
- `Limb::iterate_recursive` yöntemini herkese açık hale getirdi.
- Bir ragdoll aktif olduğunda karakterin sert gövdesini kinematiğe çevirin.
- Bir menü zinciri açarken menü öğelerini vurgulu tutun.
- Navmesh etkileşim modu için Gizmo iyileştirmeleri.
- Bir navmesh sahne düğümü seçerken sahne önizlemesinin sağ üst kısmında navmesh panelini açın.
- Bağımlılık görüntüleyicisinde görselleştirme iyileştirildi.
- Varlık içe aktarma seçenekleri denetçisi genel hale getirildi.
- Oluşturucuda malzeme bağlamına erişim sağlayın.
- Hareket, ölçek, döndürme gizmo iyileştirmeleri.
- UI düğümleri için değiştirilebilir erişim.
- Onlar için önizleme oluşturmadan önce kaynakları önceden yükleyin.
- Dünya görüntüleyicinin doğrudan sahne yerine veri sağlayıcıyı kabul etmesi sağlandı.
- `ResourceData` özelliğinde `Cow<Path>` yerine `&Path` kullanıldı.
- Malzeme düzenleyici alanında sürükle ve bırak yöntemiyle malzeme ayarlamaya izin verin.
- Denetçideki malzeme alanları daha tıklanabilir hale getirildi.
- Dize çekme algoritması kullanılarak navmesh üzerinde gezinme iyileştirildi.
- Gezinti ağı sorgularının performansı iyileştirildi.
- Metin kutusu widget performansı iyileştirildi.
- `Plane`API iyileştirmeleri.
- Malzeme düzenleyicisi varsayılan olarak biraz daha geniş hale getirildi.
- Kaynak veri yapıcısını tür adıyla genişletin.
- Malzemeyi kaynağa dönüştürdü, `SharedMaterial` yapısını kaldırdı.
- Vertex tampon verilerini bayt olarak seri hale getirin.
- `Winit` dokümanlarında önerildiği gibi `Window::pre_present_notify` kullanın.
- Materyalleri kullanmak için yeniden düzenlenmiş sprite oluşturma.
- İleri renderer kullanmak için parçacık sistemi render'ı yeniden düzenlendi.
- Aydınlatma için daha fazla yerleşik gölgelendirici değişkeni.
- Üçgen tampon API iyileştirmeleri.
- OpenGL hataları için mesaj kuyruğu yerine hata ayıklama mesajı geri çağrısı kullanın.
- Hata ayıklama derleme profilinde OpenGL hata ayıklamayı etkinleştirin.
- Geometri tamponları için özelleştirilebilir yaşam süresi (bir kare yaşayan geçici tamponlar oluşturmaya izin verir (ttl = 0)).
- Editör başlangıcında birden fazla sahne başlatmaya izin verin ([@dasimonde](https://github.com/dasimonde)'a tebrikler).
- Vertex buffer için `push_vertices` + `push_vertices_transform` yöntemi.
- ABSM editöründe bir durumu diğer tüm durumlara bağlayabilme (kudos
  'a [@Riddhiman007](https://github.com/Riddhiman007))
- Sahne düğümleri için UUID'ler eklendi.
- Navmesh aracı yolu yeniden hesaplama eşiğini ayarlayabilme.
- Düğüm türünü düzeltirken devralınabilir değişkenlerin `modified` bayraklarını sıfırlayın.
- Grafik çözümlemede düğüm tipi uyuşmazlığını kontrol edin ve bunu otomatik düzeltin.
- Kalıtım hatalarını bildirirken tür kimlikleri yerine tür adlarını kullanın.
- Grafiğin bütünlüğünü geri yüklerken öksüz düğümleri kaldırın.
- `Inspector` widget'ı için kod örneği.
- Örnek verilerini yüzeye çıkarmak için düğüm tanıtıcısını geçirin.
- Oluşturma için kameraları filtrelerken global `enabled` bayrağını kontrol edin.
- Yerel çerçeveleri bağlayan eklemleri serileştirin.
- Kullanıcı arayüzünde dokunma olayları için destek ([@Bocksdin](https://github.com/Bocksdin)'a tebrikler).
- A\* yol bulma optimizasyonu (tebrikler [@TiltedTeapot](https://github.com/TiltedTeapot)).

## Sabit

- Wayland üzerinde editörün çökmesi düzeltildi.
- Yazı tipi oluşturma API'si düzeltildi.
- Tiplenmemiş kaynakların sığ kaynak tutamaçlarının geri yüklenmesi düzeltildi.
- Değişiklikten sonra ayarların çift kaydedilmesi önlendi.
- Varlık tarayıcısında bir dokuyu önizlerken en boy oranını koruyun.
- Kaynak oluşturma iletişim kutusunda kaydedilemeyen kaynakları filtreleyin.
- Kullanıcı arayüzü düzenleyicisinde ekran dışı kullanıcı arayüzü oluşturma düzeltildi.
- Malzemelerin derin klonlanması düzeltildi: klonlamadan sonra gömülü hale getirin.
- "Uzantıları" olan klasörleri doğru şekilde işlemek için yol filtreleri düzeltildi.
- Malzeme düzenleyicide gölgelendirici özelliğini değiştirirken malzemeyi kaydedin.
- İşleyicinin her tarafına dağılmış grafik boru hattı durumuna işaretçiler içeren devasa ayak tabancası düzeltildi.
- Motor genelinde çoklu iş parçacığı havuzu oluşturulmasını önleyin.
- Bir malzeme yüklenemediğinde malzeme düzenleyicinin çökmesi düzeltildi.
- Ctrl+S ile bir sahne kaydedildikten sonra düzenleyicinin kapanması önlendi.
- Maksimize edilmiş düzenleyici penceresinin konum kaydetmesi düzeltildi.
- Bir malzeme özelliğine malzeme olmayan kaynak atandığında oluşan çökme düzeltildi.
- Kaplanmış kafesler için standart gölgelendiricinin ileri geçişi düzeltildi
- Ziyaretçideki uuid biçimlendirmesi düzeltildi.
- Düzenleyicideki kaynak uzantısı karşılaştırması büyük/küçük harfe duyarsız hale getirilerek düzeltildi.
- Sahne önizleyicisinde varlıkları sürükleyip bırakırken oluşan çökme düzeltildi.
- OpenGL hata işleme düzeltildi
- Tepe noktası/üçgen tamponlarını değiştirirken yaşanan performans sorunları düzeltildi.
- Arazileri düzenlerken oluşan çökme düzeltildi (tebrikler [@Riddhiman007](https://github.com/Riddhiman007))
- Tepe noktası öznitelik böleninin göz ardı edildiği bir hata düzeltildi.
- Günlük mesajları için renkler düzeltildi.
- Türetilmiş modda sahne yüklemesi düzeltildi.
- Metne bir `WidgetMessage::Foreground` gönderirken metin renklendirmesi düzeltildi.
- Linux'taki bellek sızıntıları düzeltildi ([@LordCocoNut](https://github.com/LordCocoNut)'a tebrikler)
- Düzenleyicide sahneler arasında geçiş yaparken grafiklerde çökmelere/sorunlara neden olan geçersiz GPU kaynak indeksleme hatası düzeltildi.

## Kaldırıldı

- Bazı türler için `Reflect::into_any` uygulamasında örtük klonlama kaldırıldı.
- Yansıma kullanarak alanları getirirken gereksiz bellek ayırma kaldırıldı.
- Alanlar üzerinde yineleme yaparken gereksiz bellek ayırma kaldırıldı.
- Kaynakların iç yapısını düzleştirmek için `typed resource`daki `Option` wrapper`ı kaldırıldı.
- Renderer'daki bir grup gereksiz klon kaldırıldı.
- Gezinme ağındaki tembel hesaplamalar kaldırıldı.
- Parçacık sistemlerinden kullanılmayan `soft_boundary_sharpness_factor` parametresi kaldırıldı (bu özellik
  standart parçacık sistemi malzemesi).
- InteractionModeKind` kaldırıldı ve uuidler ile değiştirildi.

# 0.32

- Script henüz başlatılmamışsa `Script::on_os_event` çağrısı yapmayın.
- Ziyaretçi::load_from_memory`de taşıma yerine ödünç alma.
- Sahneleri iki modda yükleyebilme - türetilmiş ve ham.
- Animasyon düzenleyicisindeki seçim sorunları düzeltildi.
- Yol düzeltici düzeltildi.
- Kaynak yolunu ayarlayabilme.
- Yüklenen kaynakların kaydını kaldırmak için `ResourceManager::unregister`.
- Yeniden düzenlenmiş sahne yükleme + eklenti arayüzü iyileştirmeleri.
- Dünya görüntüleyicide filtreyi temizlerken seçili düğümü görünüme getirin.
- Özellik seçicide arama düzeltildi.
- Filtre metni temizlenirken mevcut seçimi düğüm seçicide görünüme getirin.
- Eğri düzenleyicide tuş olmadığında 'sığdırmak için yakınlaştır' düzeltildi.
- Animasyon düzenleyicinin parça listesindeki düğüm adı biçimlendirmesi düzeltildi.
- Denetçideki araç ipuçları düzeltildi.
- Diğer güncelleme yöntemlerinden sonra çağrılan `EditorPlugin::on_post_update`.
- Düğüm seçicideki seçim senkronizasyonu düzeltildi.
- Ağaç kökü öğelerinin değiştiği anı yakalamak için `TreeRootMessage::ItemsChanged` eklendi.
- Düğüm tanıtıcısı özellik düzenleyicisinin görsel stili iyileştirildi.
- Düğüm seçici aracılığıyla sahne düğümü tanıtıcısını ayarlama yeteneği.
- `Sound::try_play` metodu, sesi sadece zaten çalmıyorsa çalacaktır.
- Doku içe aktarma seçenekleri için `Turn the green channel` seçeneği: bu, yeşil kanalları çevirme yeteneği ekler
  OpenGL Y+ biçiminde yapılmış normal haritalar.
- Kaynak yöneticisi iyileştirmeleri: şablon kodu azaltmak için otomatik uygulama ile temel özellik eklendi, zorunlu
  Kaynak yükleyici tarafından üretilen gerçek veri türünü almak için `ResourceLoader::data_type_uuid` yöntemi,
  `ResourceManager::try_request` - isteğe bağlı bir kaynak tanıtıcısı döndürür, `T` ile eşleşmezse `None` döndürür.
  gerçek veri kimliği (`request` bu durumda sadece panikler).
- Bir kaynak yüklenemediğinde günlüğe bir hata mesajı yazdırın.
- Kaynak alanı özellik düzenleyicisi iyileştirmeleri: fare seçimini iyileştirmek için şeffaf geometri yayar,
  öğeler için kenar boşlukları eklendi.
- Özel kaynak kaydedebilmek için eklenti kayıt bağlamına kaynak yöneticisi referansı eklendi
  Hem oyunda hem de editörde kullanılacak yükleyiciler.
- Malzeme özelliklerini gölgelendiricisiyle senkronize etmek için `Material::sync_to_shader` yöntemi.
- Standart gölgelendiriciler için `parallaxCenter` + `parallaxScale` özelliği.
- Mesh hata ayıklama çiziminde TBN tabanlı görselleştirme düzeltildi.
- Tüm gizmo'ların X eksenini gerçek koordinat sisteminin X ekseniyle eşleştirin.
- Varlık tarayıcısındaki araç ipucu, kırpma olmadan tam yolu gösterecek şekilde düzeltildi.
- Paralaks eşlemesi düzeltildi.
- Binormal vektör hesaplaması düzeltildi.
- Doku yükleyici için eksik `tif` uzantısı eklendi.
- Editördeki yapı penceresi çıktısı düzeltildi.
- Gölgelerin aniden ortaya çıkmasını önleyen gölgeler için solma/sönme eklendi.
- Sahne gizmosu eklendi.
- Şeffaf arka plan oluşturma için aydınlatma uygularken kare alfasını koruyun.
- Ses kaynaklarını durdurmadan önce geri sarın.
- Bir sahne nesnesine odaklanan kamera geliştirildi.
- Yörüngesel kamera kontrolleri değiştirildi: kamerayı sürükleme `Shift+RMB`ye taşındı, yapılandırılabilir yakınlaştırma aralığı eklendi.
- Editör kamerası için yörüngesel kamera modu eklendi (orta fare tuşuyla etkinleştirilebilir).
- Sahne ses kaynağının oynatma süresi için `Duration` yerine `f32` kullanın.
- Arazi fırça sınırları görselleştirmesi düzeltildi.
- Arazi düzenleyici için kısayol tuşları.
- Bağımlılık değişimini basitleştirmek için `fyrox-template` tarafından oluşturulan projelerde `workspace.dependencies` kullanın.
- Geliştirilmiş editör ayarları işleme.
- `curve::bounds` + eğri düzenleyici için gecikmeli olarak `Sığdırmak için yakınlaştırma` yeteneği.
- `curve` alanları için özellik düzenleyici.
- Yeni `fyrox-scripts` sandığı + uçan kamera denetleyici komut dosyası.
- UI tuşunu winit'e geri eşleme + tuş bağlama özelliği düzenleyicisini değiştirme yeteneği.
- Eğer `fyrox-template script` `game/src` bulamazsa kök dizine geri dönüş.
- Gpu dokusu için hata ayıklama impls eklendi.
- Arazi parçaları arasındaki dikişler düzeltildi.
- Eski örnekler kaldırıldı ve [yeni örnekler](https://github.com/FyroxEngine/Fyrox-demo-projects) ile değiştirildi.
- Kaydırma bölgeleri ile eğri düzenleyici uyumluluğu düzeltildi.
- Eğri düzenleyicideki kırpma sorunları düzeltildi.
- Dünya görüntüleyicisindeki sahne öğelerinin genişletilmiş durumu düzenleyici ayarlarına kaydedildi.
- Eğri düzenleyicide geçersiz tuşların seçilirken konumlandırılması düzeltildi.
- Eğri düzenleyicide fare dışarıdayken kutu seçimi düzeltildi.
- Animasyon düzenleyicide filtre metni temizlenirken seçili varlığa odaklanıldı.
- `CurveEditor` widget'ındaki bir dizi potansiyel çökme düzeltildi.
- WebAssembly üzerindeki HiDPI sorunları düzeltildi.
- Desteklenen kaynak uzantılarının kodlanmış listesi editörden kaldırıldı ve kaynak yükleyicilerden gelenler kullanıldı.
- HRTF ses oluşturucu için `Hrir` kaynağı + async HRTF yüklemesi.
- Doku sıkıştırması düzeltildi.
- Kötü performansa sahip olduğu için sürüm yapılarında `glCheckError` kullanmayın.
- Görüntüleyicideki kayan noktalı dokular için en yakın filtrelemeyi ayarlayın (WebAssembly düzeltmesi).
- Bazı durumlarda sonsuz yüklemeyi önlemek için yükleyicisi olmayan bir kaynağı hata durumuna geçirin.
- WebAssembly'de kaynak yüklemesi düzeltildi.
- Ekran boyutu bir noktaya daraltılırsa hiçbir şey oluşturmayın.
- Işık haritası üretimini iki adıma bölün + editör için asenkron üretim eklendi.
- Adının başında bir sayı olan oyun projeleri oluşturmaya izin vermeyin.
- Optimize edilmiş ışık haritası veri serileştirmesi (kırılma değişikliği, ışık haritalarınızı yeniden oluşturun).
- Rastgele veri kümelerini bayt olarak serileştirmek için `BinaryBlob` sarmalayıcı.
- Bir ışık haritası oluşturamadığında çökmek yerine günlüğe bir hata yazdırın.
- Işık haritasını `Scene`den `Graph` içine taşıdı.
- Bir grafiği kopyalarken ışık haritası dahili tutamaç eşlemesi düzeltildi.
- Inspector için `PathBuf` için `PathEditor` widget + özellik editörü.
- Editördeki ışık eşleyici için birim başına varsayılan texel miktarı azaltıldı.
- `fyrox-template` içinde yeni bir proje için vcs belirleme yeteneği

- `fyrox-template` tarafından oluşturulan çalışma alanları için `resolver = 2` ayarı

- Eklem hesaplama kararlılığı iyileştirildi.

- Düğüm tutamağı özellik düzenleyicisi için `Unassigned Structure` düğmesi.

- Ana düzenleyici penceresinin geçersiz pencere özniteliklerini kaydetme.

- Eklem bağlama düzeltildi.

- Eklem yeniden bağlama artık isteğe bağlıdır.

- Matristen kuaterniyon oluştururken ortaya çıkan potansiyel sonsuz döngü düzeltildi.

- Düzenleyicide grup komutuna özel ad belirleme özelliği.

- `Ragdoll` sahne düğümü.

- Düğüm tutamağı özellik düzenleyicisi için fare seçimi iyileştirildi.

- Birkaç tıklamayla ragdoll'lar oluşturmak için ragdoll sihirbazı.

- Düzenleyici için güç tasarrufu modu. Düzenleyici, penceresi odaklanmamışsa veya ana pencereden OS olayı gelmezse

çalışmasını duraklatır. Bu değişiklik, düzenleyici etkin değilken CPU/GPU kaynaklarının tüketimini sıfıra indirir.

- Değiştirilmedikleri takdirde, serileştirmede kalıtsal değişkenler için ayrı bir bölge oluşturulmaz. Bu, türetilmiş varlıklarda (kaydedilmiş oyunlar dahil) oldukça

fazla disk alanı tasarrufu sağlar.

- Kaynakların kalıtsal vec koleksiyonları için özellik düzenleyicileri.
- Ses kaynağının çalma süresini ayarlarken, giriş süresini tamponun gerçek süresine sabitleyin.
- Ogg/vorbis akış uzunluğunun alınamaması sorunu düzeltildi.

- `GenericBuffer::duration` artık hassasiyet sorunları olmayan tamsayı aritmetiği kullanıyor

(kayan nokta sayılarından farklı olarak).

- Kod çözücüler artık kanal süresini saniye olarak değil, örnek olarak döndürüyor.

- Yalnızca onay modu anında ise değişikliklerde metin kutusu mesajı gönderin.

- Editör ayarlarının yüklenememesi nedeniyle görüntülenen mesajların önem derecesi düzeltildi.

- Kaynak koleksiyonları için vec özellik editörleri eklendi.

- `Vec<T>` için özellik editörü artık `InspectablePropertyEditor`'ın örtük kullanımı yerine `T` için uygun özellik editörünü kullanacaktır

.

- Varlık tarayıcısında bir varlığın yanlış odaklanma sorunu düzeltildi.

- `TextBox` widget'ı için gönderilen mesajların yönü düzeltildi.

- Denetçide kaynak alanları için `Show in Asset Browser` düğmesi eklendi.

- HRTF renderer kullanılırken ses kaynağı kazancı dikkate alınır.
- Editörde bir yüzeyin kemik listesinin görselleştirilmesi düzeltildi.
- HRTF ses oluşturucu gecikmesi azaltıldı.
- Blend-by-index ABSM düğümleri için animasyon olayları toplama düzeltildi.
- Geliştirilmiş ABSM olay toplama API'si.
- ABSM katmanlarından animasyon olaylarını getirebilme.
- Özellik geri dönüşü düzeltildi: artık sadece değiştirilmiş olanları geri döndürüyor.
- Bir sahne düğümünün tüm devralınabilir özelliklerini bir kerede geri alma yeteneği.
- `Reflect::enumerate_fields_recursively` bir nesnenin alt alanları üzerinde yineleme yapmanızı sağlar
  her alan hakkında bilgi alırken.
- Düzenleyicide yalnızca o anda etkin olan sahneyi güncelleyin.
- Navmesh yolu yumuşatma iyileştirmeleri ve düzeltmeleri. Düzgünleştirmenin köşeleri kesmesini önleyin.
- `A*` yol bulucu API iyileştirmeleri.
- NavMesh sahne düğümü için hata ayıklama çizimi.
- Işık saçılması artık ışık yoğunluğunu hesaba katıyor.
- Aynı sahnenin birden çok kez yüklenmesini önleyin.
- Olası görsel desenkronizasyonu önlemek için sahneleri değiştirirken editördeki kullanıcı arayüzünü temizleyin.
- Geçersiz hedef widget tanıtıcısı ile kullanıcı arayüzü mesajları işlenirken olası panik düzeltildi.
- Bazı platformlarda `TextBox` widget'ında metin yazdırırken metnin iki katına çıkması düzeltildi.
- Animasyon düzenleyicisinde animasyon parçalarını çoğaltma yeteneği.
- Animasyon parçalarının kimliğini ayarlayabilme.
- Kişileri getirirken `Rapier` varlıklarının geçersiz tutamaçlarında olası panik düzeltildi.
- Farenin orta tuşunu kullanarak `TabControl` widget'ındaki sekmeleri kapatabilme.
- Yön ışıklarını editörde oklar olarak görselleştirme.
- Sahne hata ayıklama çizim bağlamında ok çizebilme.
- `Winit 0.29`a geçiş yapıldı.
- `Rect::clip_by` yöntemi düzeltildi.
- İşlevselliği standart kütüphaneye zaten eklenmiş olduğu için `VecExtensions` özelliği kaldırıldı.
- `Popup` widget geliştirmeleri: Yerleştirme::hedef` yöntemi, eklemeden açılır pencere oluşturma yeteneği
  kullanıcı arayüzüne eklendi.
- Menü' widget'ındaki potansiyel sonsuz döngü düzeltildi.
- Klasör oluşturabilmek ve dosyaları kaldırabilmek için dosya tarayıcısına içerik menüsü eklendi.
- `fyrox-core` ve `fyrox-resource` crates için test kapsamı önemli ölçüde geliştirildi (kudos to
  [@san-smith](https://github.com/san-smith))
- Bir düğüme grafiğin herhangi bir yerinde atıfta bulunulduğunda uyarı vermek için isteğe bağlı düğüm silme iletişim kutusu.
- Tepe noktası tamponundaki potansiyel çift serbest sorunu düzeltildi.
- Tepe noktası tamponundaki tip ölçümünün sağlam olmaması düzeltildi.
- Grafikteki düğüm referanslarını aramak için `Graph::find_references_to`.
- Bir nesnenin alt alanları üzerinde özyinelemeli yineleme için `Reflect::apply_recursively`.
- `fyrox-template` için `try` ayrılmış anahtar sözcüğü eklendi.
- `Kamera` sahne düğümü için yerleşik gökyüzü kutusu.
- Dünya Görüntüleyicisi'nde arama geliştirildi.
- `TriangleDefinition` üçgen olarak kopyalanabilir hale getirildi.
- Büyük UI dokümantasyon iyileştirmeleri.
- `VectorImage`, `ScrollPanel`, `RectEditor`, `RangeEditor`, `ProgressBar`, `ListView`, `Canvas` için dokümanlar,
  `SearchBar`, `ScrollViewer`, `Expander`, `KeyBindingEditor`, `HotKeyEditor`, `Tree`, widgets.
- Büyük kitap geliştirmeleri.
  **_ Translated with www.DeepL.com/Translator (free version) _**

# 0.31

- Çoklu sahne düzenleme

- `Window` widget için belgeler

- Opengl desteklenmediğinde opengl es kullanımı düzeltildi

- `Decorator` widget için belgeler

- `CurveLoader` için `crv` uzantısı eklendi

- Temel düzenleyici eklentileri desteği

- Bağımlılıklar güncellendi

- Tüm düzenleyici alanları dışarıdan erişilebilir hale getirildi
- `UuidEditor` widget'ı için belgeler

- Fizik varlıklarının user_data alanını motor varlığına ait tanıtıcıyı depolamak için kullanma

- Tanıtıcıları u128'e kodlama/kod çözme yeteneği

- 2d/3d fizik dünyalarından tüm temas çiftlerini alma yeteneği

- `MessageBox` widget'ı için belgeler

- `Graph::aabb_of_descendants`
- Aabb api iyileştirmeleri

- Dünya görüntüleyiciden bir düğümün varlığını açma yeteneği

- `field.foo.ab` zincirlerini kabul etmek için `impl_component_provider` makrosu iyileştirildi

- Navmesh düğümü için belgeler

- Davranış ağaçları için kullanışlı kısayollar

- Yeni serileştirme biçimi için standart malzemeler düzeltildi
- Davranış ağaçları için invertör düğümü

- `VertexBuffer` için belgeler ve örnekler

- Farklı düzenlere sahip köşe tipi kullanımını önlemek için `VertexTrait` eklendi

- `surface` mod belgeleri iyileştirildi

- `PluginContext` içine `elapsed_time` eklendi
- Sprite fragman gölgelendiricisinde tüm doku kanallarını kullanma

- Yeniden yapılandırma sırasında editörün kenetlenme yöneticisi düzenini yükleme

- Kenetlenme yöneticisi düzenini geri yüklerken bir döşemenin penceresini açma

- Editörün kenetlenme yöneticisi düzenini kaydetme/yükleme

- UI arama yöntemlerinde paniği önleme

- Kaydedilen kenetlenme yöneticisi düzenini uygulama + düzen kaydetme iyileştirmeleri

- Kenetlenme yöneticisi düzenini kaydetme
- Eksik seçenek dosyası yüklenemediğinde hata uyarısına dönüştürüldü

- Editörden çıkarken yaşanan çökme sorunu düzeltildi

- Varlık tarayıcısından rastgele dosyaların açılması sorunu düzeltildi

- Varlık tarayıcısından sahneleri açma özelliği eklendi

- Sekmeler için kullanıcı tanımlı veriler

- Mesajlar aracılığıyla `TabControl` widget'ında sekme ekleme ve kaldırma özelliği eklendi

- Dokuz yama widget'ı eklendi
- Sekme denetiminin içerik hizalaması düzeltildi

- `TabControl` sekmeleri için `can_be_closed` bayrağı

- `TabControl` widget'ında sekmeleri kapatma özelliği

- `TabControl` widget'ı için belgeler

- `TabControl`'ün etkin sekmesinin değiştiği anı yakalama özelliği

- `ScrollBar` widget'ı için belgeler

- `Popup` widget'ı için belgeler
- `NumericUpDown` widget'ı için belgeler

- Mesaj yoluyla `StackPanel`'in yönünü değiştirme özelliği

- Mesaj yoluyla `WrapPanel`'in yönünü değiştirme özelliği

- `WrapPanel` widget'ı için belgeler

- `CheckBox` widget'ı için belgeler

- `Widget` için belgeler

- `TextBox` widget'ı için belgeler
- `StackPanel` widget'ı için belgeler

- `Grid` widget'ı için belgeler

- `Image` widget'ı için belgeler

- `Text` widget'ı için belgeler

- Fyrox-ui belgeleri

- `Button` widget'ı için belgeler

- `TransformStack`'in geçerli dönüşümüne erişim

- `Border` için belgeler

- `define_constructor` makrosunda belge yorumlarını geçirme yeteneği
- Çoklu sahne düzenleme

- `Window` widget için belgeler

- Opengl desteklenmediğinde opengl es kullanımı düzeltildi

- `Decorator` widget için belgeler

- `CurveLoader` için `crv` uzantısı eklendi

- Temel düzenleyici eklentileri desteği

- Bağımlılıklar güncellendi

- Tüm düzenleyici alanları dışarıdan erişilebilir hale getirildi
- `UuidEditor` widget'ı için belgeler

- Fizik varlıklarının user_data alanını motor varlığına ait tanıtıcıyı depolamak için kullanma

- Tanıtıcıları u128'e kodlama/kod çözme yeteneği

- 2d/3d fizik dünyalarından tüm temas çiftlerini alma yeteneği

- `MessageBox` widget'ı için belgeler

- `Graph::aabb_of_descendants`
- Aabb api iyileştirmeleri

- Dünya görüntüleyiciden bir düğümün varlığını açma yeteneği

- `field.foo.ab` zincirlerini kabul etmek için `impl_component_provider` makrosu iyileştirildi

- Navmesh düğümü için belgeler

- Davranış ağaçları için kullanışlı kısayollar

- Yeni serileştirme biçimi için standart malzemeler düzeltildi
- Davranış ağaçları için invertör düğümü

- `VertexBuffer` için belgeler ve örnekler

- Farklı düzenlere sahip köşe tipi kullanımını önlemek için `VertexTrait` eklendi

- `surface` mod belgeleri iyileştirildi

- `PluginContext` içine `elapsed_time` eklendi
- Sprite fragman gölgelendiricisinde tüm doku kanallarını kullanma

- Yeniden yapılandırma sırasında editörün kenetlenme yöneticisi düzenini yükleme

- Kenetlenme yöneticisi düzenini geri yüklerken bir döşemenin penceresini açma

- Editörün kenetlenme yöneticisi düzenini kaydetme/yükleme

- UI arama yöntemlerinde paniği önleme

- Kaydedilen kenetlenme yöneticisi düzenini uygulama + düzen kaydetme iyileştirmeleri

- Kenetlenme yöneticisi düzenini kaydetme
- Eksik seçenek dosyası yüklenemediğinde hata uyarısına dönüştürüldü

- Editörden çıkarken yaşanan çökme sorunu düzeltildi

- Varlık tarayıcısından rastgele dosyaların açılması sorunu düzeltildi

- Varlık tarayıcısından sahneleri açma özelliği eklendi

- Sekmeler için kullanıcı tanımlı veriler

- Mesajlar aracılığıyla `TabControl` widget'ında sekme ekleme ve kaldırma özelliği eklendi

- Dokuz yama widget'ı eklendi
- Sekme denetiminin içerik hizalaması düzeltildi

- `TabControl` sekmeleri için `can_be_closed` bayrağı

- `TabControl` widget'ında sekmeleri kapatma özelliği

- `TabControl` widget'ı için belgeler

- `TabControl`'ün etkin sekmesinin değiştiği anı yakalama özelliği

- `ScrollBar` widget'ı için belgeler

- `Popup` widget'ı için belgeler
- `NumericUpDown` widget'ı için belgeler

- Mesaj yoluyla `StackPanel`'in yönünü değiştirme özelliği

- Mesaj yoluyla `WrapPanel`'in yönünü değiştirme özelliği

- `WrapPanel` widget'ı için belgeler

- `CheckBox` widget'ı için belgeler

- `Widget` için belgeler

- `TextBox` widget'ı için belgeler
- `StackPanel` widget'ı için belgeler

- `Grid` widget'ı için belgeler

- `Image` widget'ı için belgeler

- `Text` widget'ı için belgeler

- Fyrox-ui belgeleri

- `Button` widget'ı için belgeler

- `TransformStack`'in geçerli dönüşümüne erişim

- `Border` için belgeler

- `define_constructor` makrosunda belge yorumlarını geçirme yeteneği

# 0.30

- Grafik kökünü rastgele bir grafik düğümüne değiştirme yeteneği.

- Editörde grafik kökünü değiştirme yeteneği.

- `Image` widget için isteğe bağlı dama tahtası arka planı.

- Basitleştirilmiş animasyon karıştırma.

- Eğri anahtarının değerine değiştirilebilir erişim.

- Animasyon editörü için özellik doğrulama eklendi.

- Animasyon düzenleyicisi için iz doğrulama.

- Mesaj yoluyla widget'ın araç ipucunu ayarlama özelliği.

- Animasyon düzenleyicisinde iz adlarını doğru şekilde senkronize etme.

- Animasyon izlerinde hedef düğümleri değiştirme özelliği.

- Bir grafikten alt grafik çıkarırken üst öğeyi koruma.

- Kök düğümü değiştirmeye izin vermek için düzenleyici sahne yapısı yeniden düzenlendi.

- Varlık tarayıcısında incelerken ses tamponu kaynağını oynatma.

- Bir dokuyu incelerken dokulu dörtgeni kaynak önizleyicide gösterme.

- `ScrollViewer` widget'ı için yapılandırılabilir kaydırma hızı + kaydırmayı 2 kat hızlandırma.

- Kaynak durumunu hızlıca kontrol etmek için yardımcı yöntemler.

- Komut dosyası bileşenlerine daha hızlı erişmek için yardımcı yöntemler.

- Aralık özelliği düzenleyicisi iyileştirildi.

- AbsM düzenleyicisinde durum menüsü için `Enter Situation`. Çift tıklama ile aynı şekilde çalışır, çift tıklamaya alışkın olmayan kullanıcıların kafasını karıştırmaz

.

- Düzenleyicide sahneleri kapatırken veya değiştirirken önizleme modundan çıkın.

- Boş bir aralıktan rastgele sayı oluşturmaya çalışırken panik önlenir.

- Gecikme satırı örneklerini POD dizisi olarak serileştirin.

- Hata ayıklama için mevcut sahneyi metin biçiminde kaydetme seçeneği.
- Devre dışı bırakılmış sprite düğümlerini işlemeyin.

- Özellik miras alma ile ilgili küçük hatalar düzeltildi.

- Ebeveyn yoksa düzenleyicide bir özellik değerinin tersine çevrilmesine izin vermeyin.

- Değiştirilmemiş miras alınabilir değişkenlerin içeriğini kaydetmeyin.

- Yönlü ışık belgeleri düzeltildi.

- `Node::is_x,as_x,as_x_mut` yöntemleri düzeltildi.

- `Graph::try_get_script_of + try_get_script_of_mut` yöntemleri

- `Base::root_resource` - bağımlılık grafiğinde kök kaynağı bulmanızı sağlar.

- Kendine referans veren model kaynaklarında kilitlenmeyi önler

- Widget'lar için UUID.
- Editör penceresinin konumunu ve boyutunu editör ayarlarına kaydedin.

- Yükseklik alanı çarpıştırıcısına yerel arazi ölçeklendirmesini uygulayın.

- `MachineLayer::is_all_animations_of_state_ended`

- ABSM katmanındaki bir durumun tüm animasyonlarını alma yeteneği.

- ABSM geçişleri için `IsAnimationEnded` koşulu eklendi.

- ABSM durum eylemleri. Bir duruma girerken/çıkarken belirli animasyonları geri sarmanıza/etkinleştirmenize/devre dışı bırakmanıza olanak tanır.

- Hedef yerine kaynaktan gönderilen yanlış “durum girişi” olayı düzeltildi.

- Kaynak yöneticisi için yerleşik kaynakların bir koleksiyonu eklendi. Bu koleksiyon, yerleşik kaynaklara yapılan referansları geri yüklemek için kaynak serileştirme

adımında kullanılır.

- Motorun başlatılması sırasında yerleşik gölgelendiricileri önceden derleme.

- Editörde kamera yakınlaştırma hızını değiştirme yeteneği.

- `Plugin::before_rendering`

- Sürücü senkronizasyon adımlarını önlemek için matris depolama önbelleği.

- Render varlıkları için kalıcı tanımlayıcılar.

- Dizileme performansı iyileştirildi.

- Mip haritaları oluşturmak için `fast_image_resize` crate kullanın (bu, performansı 5 kat artırdı).

- Dokular için mip harita oluşturma için yapılandırılabilir filtre.

- Ara bilgi penceresinin konumu düzeltildi - artık ekran sınırlarının dışına çıkmıyor.

- Koleksiyon boyutunun değiştirilmesini önleyen koleksiyon alanları için “Değiştirilemez koleksiyon” yansıma özniteliği.

- Bir dokunun belirli mip seviyesinin türleştirilmiş verilerini alma yeteneği.

- Dokuların belirli mip seviyesi verilerini alma yeteneği.

- Arazi parçalarının yükseklik haritasını doğrudan bir görüntüden ayarlama yeteneği.

- Varlık tarayıcısı için bağımlılık grafiği görüntüleyici.

- Kaynak bağımlılık grafiği.

- Arazi eğimlerini düzleştirme özelliği.

- Işın-arazi testinde kesişme noktasında yerel yükseklik değerini döndürme.

- Editörün komut API'sı temizlendi.

- Görünürlük önbelleği kaldırıldı.

- `Handle<T: NodeTrait>` ile grafiği indeksleme özelliği

- `Handle::transmute`

- Yansıma için belge yorumları desteği.

- Seçilen varlık için belge yorumlarını ayrı bir pencerede gösterme.

- Günlük kaydediciyi `fyrox_core`'a taşındı.

- Kullanıcı tanımlı kaynakları desteklemek için kaynak sistemi yeniden düzenlendi.

- Seri hale getirme/seri hale getirmeyi kaldırma sırasında ziyaretçilerin rastgele verileri aktarması için kara tahta.

- Eksik olan arazi sınır kutusu yeniden hesaplama eklendi.

- `Texture::deep_clone`
- `Log::verify_message`
- `R32F` + `R16F` doku formatları.

- İç doku veri depolamasını belirli bir türe yeniden yorumlamak için `data_of_type` yöntemleri.

- Sahne düğümleri için hata ayıklama çizimi.

- Sahne için yapılandırılabilir poligon rasterleştirme modu (yalnızca gbuffer).

- Katı ve tel kafes renderleme arasında seçim yapmak için poligon rasterleştirme modunu ayarlama yeteneği.

- Çizilecek öğe aralığını kabul etmek için `Framebuffer::draw_x` yöntemlerini zorlama.

- Araziler için uygun culling.

- Yeniden düzenlenmiş render: sahne düğümleri artık renderer'a veri sağlayabilir. `NodeTrait::collect_render_data` artık

renderer'a veri sağlamak için kullanılır.

- Toplu oluşturma artık kamera başına (gölgeler için ışık kaynakları dahil) yapılır.

- Alt öğenin global konumunu ve dönüşünü koruyarak düğümleri bağlamak için bir yöntem eklendi.

- Araziler için LOD'lar.

- Çarpışan şekil değerleri için sınırlar.

- `Graph::begin_multi_borrow` için belge örneği eklendi.

- Farklı örnekleyici türlerine sahip malzemelerle renderlama yaparken örnekleyici türü çarpışması düzeltildi.

- Yeni bir örnekleyiciye ayarlandığında diğer örnekleyicilerden doku bağlantısı kaldırılır.

- Yarım float dokular + sabit hacimli doku mip haritaları düzeltildi.

- `RGB16F` doku formatı.

- “Sınırsız” kemik matrisleri için doku tabanlı matris depolama kullanın. Yüzey başına matris sayısını 64'ten

255'e yükseltir.

- Doku hizalama sorunları düzeltildi.

- Doku verilerini değiştirirken doğru örnekleyici indeksini kullanın.

- Verilerini değiştirirken doku için yeni mip sayısı ayarlayın.

- Doku bağlama hatası düzeltildi.

- Kemik matrisleri için yeterli alan olmadığında panik yerine uyarı verilir.

- IDE'lerde karışıklığı önlemek için `visitor::Node` adını `visitor::VisitorNode` olarak değiştirin.

- `InheritableVariable::take`

- Arazi yükseklik haritası ve katman maskelerinin boyutunu değiştirme özelliği.

- Arazinin herhangi bir tarafından parçalar ekleme özelliği.

- Navmesh kenarını silerken meydana gelen çökme sorunu düzeltildi.

- Paket açıklaması iyileştirildi.

- Navmesh paneli varsayılan olarak yüzer hale getirildi + navmesh seçildiğinde açılır.

- Navigasyon ağı yeniden düzenlendi.

- Navigasyon mesh sahne düğümü.

- Işık yoğunluğunu ışık eşlemecisine aktar.

- `Executor` için “Headless” modu - çok oyunculu oyunların sunucu tarafı için uygun.

- Editör penceresi simgesi eklendi.

- Blend shape desteği.

- Kenar çubuğu, görünüm açılır menüsünde denetleyici olarak değiştirildi.

- Dönüştürme özellikleri için adım değerleri ayarlandı.

- Vec editörü için sınırlar.

- Genel `Vector<T,N>` özellik editörü.

- VecN için min, max, step özellik öznitelikleri için destek eklendi.

- İsteğe bağlı olarak ses çıkış cihazı oluşturma/yok etme özelliği.

- Ses çıkışı arka ucu olarak `tinyaudio`'ya geçiş

- Bağlam menüleri için `RcUiNodeHandle` kullanın. Bu, bağlam menüsünün artık kullanılmadığında yok edilmesini sağlar

.

- Birden fazla ışık eşleme sorunu düzeltildi.

- WASM için yanlış `sRGB` dönüştürme düzeltildi.

- Android desteği.

- Bu parçaları isteğe bağlı hale getirerek motoru grafik/pencere/ses olmadan çalıştırma yeteneği.

- En son `winit` + `glutin` sürümüne güncelleme.

- Sürükleyerek `NumericUpDown` widget'ında değeri değiştirme

- Dünya görüntüleyiciden “Sahne Grafiği” öğesi kaldırıldı + ekmek kırıntıları çok daha kompakt hale getirildi.

- Etkileşim modu paneli sahne önizleyicinin üstüne yerleştirildi.

- Varlık tarayıcısında varlıkları arama özelliği eklendi.

- `SearchBar` widget'ı.

- Dosya tarayıcı widget'ında yol metin kutusunu gizleme özelliği.

- Varlık tarayıcısında yol alanını gizleme.

- Varlık tarayıcısında varlık öğeleri için tam varlık yolunu gösteren araç ipucu.

- Basit araç ipucu stili iyileştirildi.

- Boş olmayan menüye tıklayarak menülerin kapanmasını engelleme seçeneği.

- Editörde `No Scene` hatırlatıcısı ve sahne oluşturma/yükleme yöntemi eklendi.
- `R32F` + `R16F` doku formatları.

- İç doku veri depolamasını belirli bir türe yeniden yorumlamak için `data_of_type` yöntemleri.

- Sahne düğümleri için hata ayıklama çizimi.

- Sahne için yapılandırılabilir poligon rasterleştirme modu (yalnızca gbuffer).

- Katı ve tel kafes renderleme arasında seçim yapmak için poligon rasterleştirme modunu ayarlama yeteneği.

- Çizilecek öğe aralığını kabul etmek için `Framebuffer::draw_x` yöntemlerini zorlama.

- Araziler için uygun culling.

- Yeniden düzenlenmiş render: sahne düğümleri artık renderer'a veri sağlayabilir. `NodeTrait::collect_render_data` artık

renderer'a veri sağlamak için kullanılır.

- Toplu oluşturma artık kamera başına (gölgeler için ışık kaynakları dahil) yapılır.

- Alt öğenin global konumunu ve dönüşünü koruyarak düğümleri bağlamak için bir yöntem eklendi.

- Araziler için LOD'lar.

- Çarpışan şekil değerleri için sınırlar.

- `Graph::begin_multi_borrow` için belge örneği eklendi.

- Farklı örnekleyici türlerine sahip malzemelerle renderlama yaparken örnekleyici türü çarpışması düzeltildi.

- Yeni bir örnekleyiciye ayarlandığında diğer örnekleyicilerden doku bağlantısı kaldırılır.

- Yarım float dokular + sabit hacimli doku mip haritaları düzeltildi.

- `RGB16F` doku formatı.

- “Sınırsız” kemik matrisleri için doku tabanlı matris depolama kullanın. Yüzey başına matris sayısını 64'ten

255'e yükseltir.

- Doku hizalama sorunları düzeltildi.

- Doku verilerini değiştirirken doğru örnekleyici indeksini kullanın.

- Verilerini değiştirirken doku için yeni mip sayısı ayarlayın.

- Doku bağlama hatası düzeltildi.

- Kemik matrisleri için yeterli alan olmadığında panik yerine uyarı verilir.

- IDE'lerde karışıklığı önlemek için `visitor::Node` adını `visitor::VisitorNode` olarak değiştirin.

- `InheritableVariable::take`

- Arazi yükseklik haritası ve katman maskelerinin boyutunu değiştirme özelliği.

- Arazinin herhangi bir tarafından parçalar ekleme özelliği.

- Navmesh kenarını silerken meydana gelen çökme sorunu düzeltildi.

- Paket açıklaması iyileştirildi.

- Navmesh paneli varsayılan olarak yüzer hale getirildi + navmesh seçildiğinde açılır.

- Navigasyon ağı yeniden düzenlendi.

- Navigasyon mesh sahne düğümü.

- Işık yoğunluğunu ışık eşlemecisine aktar.

- `Executor` için “Headless” modu - çok oyunculu oyunların sunucu tarafı için uygun.

- Editör penceresi simgesi eklendi.

- Blend shape desteği.

- Kenar çubuğu, görünüm açılır menüsünde denetleyici olarak değiştirildi.

- Dönüştürme özellikleri için adım değerleri ayarlandı.

- Vec editörü için sınırlar.

- Genel `Vector<T,N>` özellik editörü.

- VecN için min, max, step özellik öznitelikleri için destek eklendi.

- İsteğe bağlı olarak ses çıkış cihazı oluşturma/yok etme özelliği.

- Ses çıkışı arka ucu olarak `tinyaudio`'ya geçiş

- Bağlam menüleri için `RcUiNodeHandle` kullanın. Bu, bağlam menüsünün artık kullanılmadığında yok edilmesini sağlar

.

- Birden fazla ışık eşleme sorunu düzeltildi.

- WASM için yanlış `sRGB` dönüştürme düzeltildi.

- Android desteği.

- Bu parçaları isteğe bağlı hale getirerek motoru grafik/pencere/ses olmadan çalıştırma yeteneği.

- En son `winit` + `glutin` sürümüne güncelleme.

- Sürükleyerek `NumericUpDown` widget'ında değeri değiştirme

- Dünya görüntüleyiciden “Sahne Grafiği” öğesi kaldırıldı + ekmek kırıntıları çok daha kompakt hale getirildi.

- Etkileşim modu paneli sahne önizleyicinin üstüne yerleştirildi.

- Varlık tarayıcısında varlıkları arama özelliği eklendi.

- `SearchBar` widget'ı.

- Dosya tarayıcı widget'ında yol metin kutusunu gizleme özelliği.

- Varlık tarayıcısında yol alanını gizleme.

- Varlık tarayıcısında varlık öğeleri için tam varlık yolunu gösteren araç ipucu.

- Basit araç ipucu stili iyileştirildi.

- Boş olmayan menüye tıklayarak menülerin kapanmasını engelleme seçeneği.

- Editörde `No Scene` hatırlatıcısı ve sahne oluşturma/yükleme yöntemi eklendi.

# 0.29

- Animasyon sistemi yeniden çalışıldı.

- Animasyon Editörü.

- Animasyon Karıştırma Durum Makinesi Editörü.

- Bağlantılı katı cisimlerden önce eklem başlatıldığında meydana gelen olası çökme sorunu düzeltildi.

- Model örneklendirme ölçeklendirme artık prefab önizleme için kullanılıyor.

- Perspektif ve orto projeksiyonlarda birçok olası panik kaynağı düzeltildi.

- 3D modu için editörün kamera hareket hızı ayarı düzeltildi.

- Standart “iki taraflı” gölgelendirici - yapraklar ve çimler için kullanışlıdır.

- Sprite sayfası editörü

- Dizicideki `Vector(2/3/4)<f32/f64/u8/i8/u16/i16/u32/i32/u64/i64>` türleri için destek.

- Sprite sayfası animasyonu artık her kare için açık uv dikdörtgenleri yerine kare koordinatlarını kullanıyor.

- Sprite sayfası animasyonuna artık bir doku eşleştirildi.

- Alan ayarlayıcısının eksik olması durumunda yansıma geri dönüşü düzeltildi.

- Image widget için uv dikdörtgeni ayarlama özelliği

- Editör için sahne ayarları penceresi - sahne ayarlarını düzenleme olanağı sağlar:

fizik entegrasyon parametrelerini, ortam aydınlatma rengini, çeşitli bayrakları vb. değiştirin.

- Editörde Mesh düğmesine yeni bir yüzey eklerken çökmeyi önleyin

- Bir öğeye çift tıkladığında dosya tarayıcı widget'ında dizin/dosya yinelemeleri düzeltildi.

- Denetçi'de malzemelerin kullanım sayısını göster

- `Arc<Mutex<Material>>` öğesini `SharedMaterial` yeni türüyle değiştir.

- Bir malzemenin benzersiz bir kopyasını bir nesneye atama yeteneği.

- `Arc<Mutex<Material>>` öğesini `SurfaceSharedData` ile değiştir

- Serileştirme öncesinde koleksiyonları temizle

- Koleksiyonlar için özellik mirası

- Özel malzemeler içeren bir FBX ile sahne yüklenirken yanlış malzeme değiştirme sorunu düzeltildi.

- FBX yükleyiciye Blender malzeme yuvaları adları eklendi

- `SurfaceData` için `procedural` bayrağına erişim

- Mesh'in yüzey verileri için özellik düzenleyici.

- Sahne düğümleri için doğrulama

- Aşağıdaki gibi geçersiz durumları bulmaya yardımcı olur:
- Eksik bağlantı gövdeleri veya geçersiz gövde türleri (örneğin, 3B bağlantı için 2B rijit gövde kullanılması)

- Yanlış eklenmiş çarpışmalar (rijit gövdenin alt öğesi olmaması)

- Bir düğümde bir sorun varsa küçük ünlem işareti gösterilir

- Klonlama sırasında widget'lar arasında araç ipucu paylaşımı

- Renk seçici düzeltildi: parlaklık-doygunluk ızgarası görünmüyordu

- Çarpışan nesnelerin kesişim kontrolü için destek eklendi (teşekkürler [@Thomas Hauth](https://github.com/ThomasHauth))

- Animasyon sistemi yeniden düzenlendi

- Sayısal özellikler için eğriler kullanın.

- Yansıma yoluyla rastgele sayısal özellikleri canlandırma yeteneği.

- Animasyonda geçersiz düğüm tanıtıcısı durumunda çökmeyi önleme

- `Curve::value_at` optimizasyonu - Aralıklar için ikili arama kullanarak 2 kat performans artışı.

- `Curve::add_key` ikili arama kullanarak ekleme optimizasyonu.

- Düğüm Seçici widget - sahneden bir düğüm seçmenizi sağlar.

- `Inspect` özelliğini `Reflect` özelliğine birleştirin - artık alanları yinelerken alan meta verilerini elde etmek mümkündür

- Özellik Seçici widget - `Reflect` özelliğini destekleyen bir nesneden bir özellik yolu seçmenizi sağlar.

- `Uuid` için `Reflect` uygulaması

- `fyrox::gui::utils::make_cross` - bir haç vektör görüntüsü oluşturmak için küçük yardımcı program

- `FieldInfo::type_name` - kararsız

`std::any::type_name_of_val`

kullanmadan bir alanın tür adını almayı sağlar - A\* yol bulma için `PathVertex::g_score` cezası (teşekkürler [@cordain](https://github.com/Cordain))

- `RawMesh` için `Default`, `Debug`,`Clone` impl'leri eklendi

- `Curve` için isim ve uuid

- `CurveEditor` widget'ına yeni anahtarlar eklendiğinde eğri gönder

- Eğri düzenleyici widget'ında eğri ve anahtar kimliklerini koru

- `Audio Panel`'i yerleştirme yöneticisi döşemesinde doğru şekilde sar (teşekkürler [@iRaiko](https://github.com/iRaiko))

- `AsyncSceneLoader` - çapraz platform (wasm dahil) asenkron sahne yükleyici

- Fyrox-template'de wasm desteği eklendi - artık fyrox-template, webassembly için özel bir

executor sürümü olan `executor-wasm` crate'i oluşturuyor

- Sahne komut dosyalarını işlemeden önce engellemeyen kaynak bekleme

- Ses durumu için eksik özellik düzenleyicisi eklendi

- Önce ses arabelleğini senkronize et, ardından oynatma konumunu

- `Machine` türü için özellik editörü.

- `VectorImage` widget'ı için Rectangle+RectangleFilled ilkelleri

- Görünümün üst kısmındaki eğri editörü widget'ında x değerlerini çiz

- Eğri editörü widget'ında eksen değerlerini gösterme/gizleme özelliği

- Eğri editöründe görünüm konumunu değiştirmek ve yakınlaştırmak için mesajlar kullan (yakınlaştırma veya görünüm

konumu değiştiğinde anı yakalamaya yardımcı olur)

- Çerçeve sırasında ne zaman oluştuklarına bağlı olarak eklentilere iletilmeyen UI mesajları düzeltildi (teşekkürler

[@bolshoytoster](https://github.com/bolshoytoster))

- Uzunluk yerine animasyon zaman dilimini açıkça ayarlama yeteneği.

- Bir düğümün klonlanması artık tam bir klon oluşturur.

- Numericupdown widget için min, max değerleri, adım ve hassasiyet ayarlama özelliği

- Yansıma kullanarak havuz öğeleri üzerinde yineleme yapmaya çalışırken paniği önleme

- `Model::retarget_animations` iki ayrı yönteme bölünmesi

- Move gizmo için akıllı hareket (teşekkürler [@Zoltan Haindrich](https://github.com/kgyrtkirk))

- `Reflect::set_field_by_path`

- `CurveEditor`'da vurgulamak için bölgeler ekleme özelliği

- `CurveEditor` widget'ında yakınlaştırma sırasında shift veya ctrl tuşlarına basarak düzgün olmayan yakınlaştırma yapma özelliği

- Animasyon sinyalleri yeniden düzenlendi

- Sayısal tanımlayıcı yerine uuid

- Sinyaller için isim eklendi

- Alıcılar/ayarlayıcılar kaldırıldı

- Daha fazla sinyal yönetimi yöntemi eklendi

- `Animation::pop_signal`

- Animasyon katmanlarını desteklemek için animasyon karıştırma durum makinesi yeniden düzenlendi
- `HashSet` için `Visit` impl

- absm düzenleyicide katman maskesi ayarlama özelliği

- Animasyon sistemi belgeleri eklendi.

- `Graph::try_get_of_type+try_get_mut_of_type`

- Belirsizliği gidermek için `InheritableVariable` yöntemlerinin adı değiştirildi

- `Model::retarget_animations_to_player`

- `PoseWeight` için doğru özellik editörünü kullan
- Editörde absm varlıklarının tutamaçlarını göster

- absm düğümleri hakkında daha fazla bilgi göster

- PlayAnimation düğümleri animasyonun adını göster

- blend düğümleri karıştırılan animasyonların miktarını göster

- `AnimationContainer::find_by_name_ref/mut`

- Çeşitli animasyon varlıklarını adlarına göre arama yeteneği

- `fyrox-template` içindeki panik mesajlarına daha fazla bilgi ekleyin (teşekkürler [@lenscas](https://github.com/lenscas))

- `fyrox-template` içinde ayrılmış isimleri kontrol edin (teşekkürler [@TheEggShark](https://github.com/TheEggShark))

- Sahne düğümlerini etkinleştirme/devre dışı bırakma özelliği

- Oyunların sunucu kısmı için başsız mod için temel destek (teşekkürler [@martin-t](https://github.com/martin-t))

- `Scene::remove_node` kaldırıldı

- `NodeTrait::clean_up` -> `NodeTrait::on_removed_from_graph` olarak yeniden adlandırıldı

- Dünya görüntüleyicide renklendirme düzeltildi

- Grafiğin güncelleme ardışık düzeninin adımlarını devre dışı bırakma özelliği

- Animasyon oynatıcı, animasyon karıştırma durum makinesi ve parçacık sistemi düğümleri için önizleme modu.

- Çarpışan `ParticleSystem::set_enabled` yönteminin adı `play` olarak değiştirildi

- Parçacık sistemi önizleme kontrol paneli

- `Uuid` türü için özellik düzenleyici.

- `Reflect` özelliğini `Debug` ile sınırlandır.

- `Inspector` widget'ındaki özellikler için `Değeri Dize Olarak Kopyala` seçeneği

- Animasyon sinyali adını animasyon olayına aktar - aynı ada sahip birden fazla animasyon olayına yanıt vermeyi çok daha kolay hale getirir

-

UI pencerelerini maksimize etme özelliği

- `Animation::take_events`

- `Reflect::type_name`

- Seçilen nesnenin tür adını denetçide göster

- Dünya görüntüleyicide birden fazla düğümün ebeveynlik sorunu düzeltildi

- Prefab oluştururken ızgara yapıştırma özelliği eklendi

- Ağaç widget'ı için aralık seçimi eklendi (Shift + Tıklama)

- Yerleştirme yöneticisi, yerleştirilmiş bir pencereyi kapatırken artık döşemeleri daraltıyor

- Dünya görüntüleyicide arama çubuğu stili iyileştirildi

- Dünya görüntüleyicide ekmek kırıntıları iyileştirildi

- `HotKey` + `KeyBinding` + ilgili özellik düzenleyicileri

- Düzenleyici kontrollerini değiştirme özelliği.

# 0.28

- Prefab oluşturma önizlemesi.

- Önizleme düğümleri artık giriş açısından şeffaftır.

- Çift tıklayarak ağaçları genişletin/daraltın.

- Fare olayları için taşıma/döndürme/ölçeklendirme gizmo davranışı düzeltildi.

- Editörün yapılandırması bozulduğunda varsayılanlara geri dönme sorunu düzeltildi.

- Editörün yapılandırmasında `Seçimleri Kaydet` seçeneği eklendi.

- Editörde sahne değiştirildiğinde ekmek kırıntıları temizlenir.

- Editörde 1 kare gecikme sorunu düzeltildi.

- Bırakma mesajından önce Fare Bırakma mesajı gönderilir.

- Bazı durumlarda editördeki UI “yanıp sönmesi” sorunu düzeltildi.

- Zaten silinmiş düğümlerden gelen UI mesajları sessizce atılmasın.

- Düğüm tanıtıcıları editördeki ekmek kırıntılarında gösterilsin.

- UI'da mevcut sürükleme bağlamına doğrudan salt okunur erişim sağlanın.

- Editörde geçersiz tanıtıcıyla bir düğüm seçilmeye çalışıldığında çökme sorunu düzeltildi.

- Geçersiz tanıtıcılar Denetçi'de vurgulanın.

- Editörde eylemleri geri alırken “kalan” hata ayıklama geometrisini atın.
- Editördeki bazı menüler artık daha sezgisel.

- RGB dokular için yanlış paket açma hizalamasıyla ilgili kritik hata düzeltildi - bu, bazı

durumlarda sert çökmelere neden oluyordu.

- Zaten yüklenmekte olan bir kaynağı yeniden yüklemeye çalışmayın.

- `Executor` için istenen kare hızını ayarlama özelliği (varsayılan 60 FPS).

- Editörün panosundaki içeriği seçilen düğüme yapıştırma özelliği (alt öğe olarak yapıştırma işlevi).

- Dokunulmamış piksellerin şeffaflığını koruyarak şeffaf pencereye render etme özelliği (bkz.

`transparent` örneği).

- `Executor`'da özel pencere oluşturucu belirleme özelliği + `Executor`'da vsync'i devre dışı bırakma yöntemi.

- `Pool` ve `Graph::begin_multi_borrow` için `MultiBorrowContext`, farklı öğelere birden fazla değiştirilebilir

referans ödünç almanıza yardımcı olur.

- Proc-makrolarda kod oluşturma hızı artırıldı.

- Özellik mirasından sonra örneklerdeki tanıtıcıları doğru şekilde eşler (komut dosyalarındaki düğümlere ait tanıtıcıların

yanlış tanıtıcılarla eşlenmesi sorunu giderildi)

- Komut dosyası işleme yeniden düzenlendi:

- `ScriptTrait::on_start` eklendi - sahnedeki tüm komut dosyaları başlatıldıktan sonra çağrılması garanti edilir, bir komut dosyası başka bir komut dosyasına bağlı olduğunda kullanışlıdır

- - Komut dosyası işleme artık merkezi hale getirildi, eskisi gibi dağınık değil.

  - Daha belirleyici güncelleme yolu (`on_init` -> `on_start` -> `on_update` -> `on_destroy`)

- Mesaj yoluyla bir metin kutusundaki metni değiştirdikten sonra bir şey yazmaya çalıştığında meydana gelen çökme düzeltildi.

- `ButtonBuilder::with_text_and_font`

- Düzenleyicide yapıların `Handle<Node>` alanlarının alanlarında düğüm adlarını gösterir.

- Bir komut dosyasında kaynak alanı olduğunda düzenleyicide meydana gelen çökme sorunu düzeltildi.

- Davranış ağaçlarını klonlama özelliği eklendi.

- Yansıma yoluyla otomatik düğüm tanıtıcı eşleme.

- `ScriptTrait::remap_handles` yöntemi kaldırıldı.

- Geçen süreyi komut dosyalarına aktar.

- Sahne devre dışıysa `ScriptTrait::on_os_event` çağrısı yapma.

- Dünya görüntüleyicinin filtrelemesinde büyük/küçük harf duyarlılığını kaldır.

- Grafiğin kök düğümü için kendi tanıtıcı ve göndereni doğru şekilde ayarla.

- “Sıcak” yöntemler için `#[inline]` öznitelikleri.
- `HashSet` için `Visit` impl

- absm düzenleyicide katman maskesi ayarlama özelliği

- Animasyon sistemi belgeleri eklendi.

- `Graph::try_get_of_type+try_get_mut_of_type`

- Belirsizliği gidermek için `InheritableVariable` yöntemlerinin adı değiştirildi

- `Model::retarget_animations_to_player`

- `PoseWeight` için doğru özellik editörünü kullan
- Editörde absm varlıklarının tutamaçlarını göster

- absm düğümleri hakkında daha fazla bilgi göster

- PlayAnimation düğümleri animasyonun adını göster

- blend düğümleri karıştırılan animasyonların miktarını göster

- `AnimationContainer::find_by_name_ref/mut`

- Çeşitli animasyon varlıklarını adlarına göre arama yeteneği

- `fyrox-template` içindeki panik mesajlarına daha fazla bilgi ekleyin (teşekkürler [@lenscas](https://github.com/lenscas))

- `fyrox-template` içinde ayrılmış isimleri kontrol edin (teşekkürler [@TheEggShark](https://github.com/TheEggShark))

- Sahne düğümlerini etkinleştirme/devre dışı bırakma özelliği

- Oyunların sunucu kısmı için başsız mod için temel destek (teşekkürler [@martin-t](https://github.com/martin-t))

- `Scene::remove_node` kaldırıldı

- `NodeTrait::clean_up` -> `NodeTrait::on_removed_from_graph` olarak yeniden adlandırıldı

- Dünya görüntüleyicide renklendirme düzeltildi

- Grafiğin güncelleme ardışık düzeninin adımlarını devre dışı bırakma özelliği

- Animasyon oynatıcı, animasyon karıştırma durum makinesi ve parçacık sistemi düğümleri için önizleme modu.

- Çarpışan `ParticleSystem::set_enabled` yönteminin adı `play` olarak değiştirildi

- Parçacık sistemi önizleme kontrol paneli

- `Uuid` türü için özellik düzenleyici.

- `Reflect` özelliğini `Debug` ile sınırlandır.

- `Inspector` widget'ındaki özellikler için `Değeri Dize Olarak Kopyala` seçeneği

- Animasyon sinyali adını animasyon olayına aktar - aynı ada sahip birden fazla animasyon olayına yanıt vermeyi çok daha kolay hale getirir

-

UI pencerelerini maksimize etme özelliği

- `Animation::take_events`

- `Reflect::type_name`

- Seçilen nesnenin tür adını denetçide göster

- Dünya görüntüleyicide birden fazla düğümün ebeveynlik sorunu düzeltildi

- Prefab oluştururken ızgara yapıştırma özelliği eklendi

- Ağaç widget'ı için aralık seçimi eklendi (Shift + Tıklama)

- Yerleştirme yöneticisi, yerleştirilmiş bir pencereyi kapatırken artık döşemeleri daraltıyor

- Dünya görüntüleyicide arama çubuğu stili iyileştirildi

- Dünya görüntüleyicide ekmek kırıntıları iyileştirildi

- `HotKey` + `KeyBinding` + ilgili özellik düzenleyicileri

- Düzenleyici kontrollerini değiştirme özelliği.

# 0.28

- Prefab oluşturma önizlemesi.

- Önizleme düğümleri artık giriş açısından şeffaftır.

- Çift tıklayarak ağaçları genişletin/daraltın.

- Fare olayları için taşıma/döndürme/ölçeklendirme gizmo davranışı düzeltildi.

- Editörün yapılandırması bozulduğunda varsayılanlara geri dönme sorunu düzeltildi.

- Editörün yapılandırmasında `Seçimleri Kaydet` seçeneği eklendi.

- Editörde sahne değiştirildiğinde ekmek kırıntıları temizlenir.

- Editörde 1 kare gecikme sorunu düzeltildi.

- Bırakma mesajından önce Fare Bırakma mesajı gönderilir.

- Bazı durumlarda editördeki UI “yanıp sönmesi” sorunu düzeltildi.

- Zaten silinmiş düğümlerden gelen UI mesajları sessizce atılmasın.

- Düğüm tanıtıcıları editördeki ekmek kırıntılarında gösterilsin.

- UI'da mevcut sürükleme bağlamına doğrudan salt okunur erişim sağlanın.

- Editörde geçersiz tanıtıcıyla bir düğüm seçilmeye çalışıldığında çökme sorunu düzeltildi.

- Geçersiz tanıtıcılar Denetçi'de vurgulanın.

- Editörde eylemleri geri alırken “kalan” hata ayıklama geometrisini atın.
- Editördeki bazı menüler artık daha sezgisel.

- RGB dokular için yanlış paket açma hizalamasıyla ilgili kritik hata düzeltildi - bu, bazı

durumlarda sert çökmelere neden oluyordu.

- Zaten yüklenmekte olan bir kaynağı yeniden yüklemeye çalışmayın.

- `Executor` için istenen kare hızını ayarlama özelliği (varsayılan 60 FPS).

- Editörün panosundaki içeriği seçilen düğüme yapıştırma özelliği (alt öğe olarak yapıştırma işlevi).

- Dokunulmamış piksellerin şeffaflığını koruyarak şeffaf pencereye render etme özelliği (bkz.

`transparent` örneği).

- `Executor`'da özel pencere oluşturucu belirleme özelliği + `Executor`'da vsync'i devre dışı bırakma yöntemi.

- `Pool` ve `Graph::begin_multi_borrow` için `MultiBorrowContext`, farklı öğelere birden fazla değiştirilebilir

referans ödünç almanıza yardımcı olur.

- Proc-makrolarda kod oluşturma hızı artırıldı.

- Özellik mirasından sonra örneklerdeki tanıtıcıları doğru şekilde eşler (komut dosyalarındaki düğümlere ait tanıtıcıların

yanlış tanıtıcılarla eşlenmesi sorunu giderildi)

- Komut dosyası işleme yeniden düzenlendi:

- `ScriptTrait::on_start` eklendi - sahnedeki tüm komut dosyaları başlatıldıktan sonra çağrılması garanti edilir, bir komut dosyası başka bir komut dosyasına bağlı olduğunda kullanışlıdır

- - Komut dosyası işleme artık merkezi hale getirildi, eskisi gibi dağınık değil.

  - Daha belirleyici güncelleme yolu (`on_init` -> `on_start` -> `on_update` -> `on_destroy`)

- Mesaj yoluyla bir metin kutusundaki metni değiştirdikten sonra bir şey yazmaya çalıştığında meydana gelen çökme düzeltildi.

- `ButtonBuilder::with_text_and_font`

- Düzenleyicide yapıların `Handle<Node>` alanlarının alanlarında düğüm adlarını gösterir.

- Bir komut dosyasında kaynak alanı olduğunda düzenleyicide meydana gelen çökme sorunu düzeltildi.

- Davranış ağaçlarını klonlama özelliği eklendi.

- Yansıma yoluyla otomatik düğüm tanıtıcı eşleme.

- `ScriptTrait::remap_handles` yöntemi kaldırıldı.

- Geçen süreyi komut dosyalarına aktar.

- Sahne devre dışıysa `ScriptTrait::on_os_event` çağrısı yapma.

- Dünya görüntüleyicinin filtrelemesinde büyük/küçük harf duyarlılığını kaldır.

- Grafiğin kök düğümü için kendi tanıtıcı ve göndereni doğru şekilde ayarla.

- “Sıcak” yöntemler için `#[inline]` öznitelikleri.
- Sert cisim bir sahnenin kök düğümü olduğunda ortaya çıkan panik sorunu düzeltildi.

- `Base::has_script` + `Base::try_get_script` + `Base::try_get_script_mut` yardımcı yöntemleri, artık sahne düğümlerindeki komut dosyalarını almak daha kolay

.

- Editörde seçilen düğüm türünü değiştirme yeteneği (sahne kök türünü değiştirmek için kullanışlıdır).

- Komut dosyası özelliği parametre aktarımı optimize edildi, komut dosyası bağlamı artık değer yerine referans olarak aktarılır.

- Komut dosyası bağlamı artık tüm eklentilere erişebilir, bu da eklentiler arası etkileşimi mümkün kılar.

- Komut dosyası API'sının üst eklentinin uuid'sini sağlaması gerekliliği kaldırıldı.

- Eklentiler için uuid tanımlamaya artık gerek yoktur.

- Devre dışı bırakılmışsa sahne komut dosyalarını güncellemeyin.

- `Graph::find_first_by_script` - komut dosyası türüne göre bir düğümü bulmanıza yardımcı olur.

- `Inspector` widget'ı için eksik özellik düzenleyicileri eklendi.

- Editörün sahne kamera ayarlarını (konum, yön, yakınlaştırma vb.) sahne başına kaydedin.

- Bazı karakterleri boşluk gibi işlemek için karakter atlama listesi.

- İsteğe bağlı metin gölge efekti.

- Metin kutusu widget'ında metni kelime kelime seçmek için Ctrl+Shift+Ok tuşlarını kullanın.

- Editörün ayar paneline navmesh ayarları eklendi.

- Metin kutusu widget'ının metin mesajlarını + metin kutusu widget'ı için özel mesajları kabul etmesini sağlayın.

- 500 ms çift tıklama aralığı ayarlayın (önceden 750 ms idi).

- Özel UI ölçeklendirme durumunda metin seçimi düzeltildi.

- `TextBox::screen_pos_to_text_pos` düzeltildi - `char_code`'un indeks olarak yanlış kullanılması, ekran

konumunun metin konumuna yanlış eşlenmesine neden oluyordu.

- Metin kutusu widget'ında metni kaydırma özelliği.

- `Rect::with_position` + `Rect::with_size` yöntemleri.

- Belirli durumlarda metin kutusundan metin silinirken imleç konumunun düzeltilmesi.

- Kelime sarma özelliği etkinleştirildiğinde metin kutusunun sonuna boşluk yazıldığında yaşanan çökme sorunu düzeltildi.

- Çok satırlı metin kutusu widget'ına metin eklenirken imleç konumunun düzeltilmesi.

- Metin kutusu widget'ında yeni satır eklenmesi sorunu düzeltildi.

- Metin kutusunda çift tıklama ile kelimeleri (veya boşlukları) seçme özelliği eklendi.

- Fare tuşu basıldıktan sonra (önce değil) çift tıklama gönderilir.

- Çeşitli senaryolarda metin kutusu widget'ında karet yanıp sönmesi düzeltildi.

- Metin kutusu widget'ında kelimeleri atlamak için Ctrl+Sol Ok ve Ctrl+Sağ Ok tuşları eklendi.

- Satır sınırlarının dışına tıklayarak metin kutusu widget'ında karet konumunu ayarlama özelliği eklendi.

- `raycast2d` örneği.

- `Delete` tuşu + seçim düzeltmeleri ile metin kutusundaki metin silme sorunu düzeltildi.

- Metin kutusu widget'ında Ctrl+Home, Ctrl+End ile seçim sorunu düzeltildi.

- Metin kutusu widget'ında seçilen metnin vurgulanması sorunu düzeltildi.

- Seçim sağdan sola olduğunda metin kutusunda Ctrl+C tuşlarına basıldığında ortaya çıkan panik sorunu düzeltildi.

- Mesaj göndererek widget'a odaklanma/odaklanmayı kaldırma özelliği eklendi.

- `TextBox` örneği eklendi.

- `PropertyInfo`'dan `is_modified` bayrağı kaldırıldı.

- Kalıtsal özelliği ebeveynin prefab değerine geri döndürme özelliği eklendi.

- Manuel özellik kalıtımı yansıma ile değiştirildi.

- `Reflect` özelliği için `fields` ve `fields_mut` eklendi.

- Komut dosyaları için özellik kalıtımı.

- Seçimi prefab olarak çıkarma özelliği eklendi.

- `Inspector` widget'ındaki karmaşık özellikler için araç ipuçları düzeltildi.

- Editörden oyun çalıştırılırken yapı profili seçilebilme özelliği eklendi.

- `Inspector` widget'ının bazı sınırlamalarını aşmak için `NodeHandle` sarmalayıcı eklendi.

- `make_relative_path` içinde açma ve panik yerine sonuç döndürülür - editördeki sembolik bağlantılarla ilgili bazı sorunlar düzeltildi.

- `fyrox-template` ile oluşturulan komut dosyaları için eksik `Reflect` uygulaması eklendi.

- `fyrox-template` ile oluşturulan projeler için bağımlılık optimizasyonu eklendi.

- Eklentilere bazı ses motoru yöntemlerine erişim sağlandı (`set_sound_gain` ve `sound_gain`)

- ArrayPropertyEditor widget'ının stili düzeltildi.

- Devre dışı bırakılmış animasyon sinyalleri için olay yayma.

- Sinyalleri olan sprite sheet animasyonları.

- Arazi renderleme düzeltildi - skybox içeriği olan katmanlar arasında artık ek yerleri yok.

- Renderer'da çizim parametrelerinde karıştırma denklemi ayarlama özelliği.

- Renderer'da RGB ve Alpha için ayrı ayrı karıştırma işlevi ayarlama özelliği.

- Sınır dışı tıklama ile menü zincirini kapatırken görünmez menüleri yok sayma.

- Editördeki bazı düğmelerin boyutu küçültülerek parlaklığı azaltıldı, daha fazla araç ipucu eklendi.

- Dünya görüntüleyicide `Expand all, Collapse all, Find selection` düğmeleri için resimler kullanıldı.

- Bazı matematik işlemleri sırasında ortaya çıkan potansiyel sonsuz döngüler düzeltildi.

- Basamaklı gölge haritaları için yumuşatma eklendi.

- Komut dosyası özelliği editörü düzeltildi - komut dosyalarını bir düğümden ayarlarken/düzenlerken/silerken editörde garip hatalar artık görülmüyor.

- Yönlü ışıklar için basamaklı gölge haritaları düzeltildi.

- `Frustum::center` yöntemi eklendi.

- Editördeki `Appearance` menüsündeki panel listesi düzeltildi.

- Varsayılan olarak gizlenmiş etkileşim modları için araç ipuçları oluşturuldu.

- Editör yeniden yapılandırıldığında ayarlar yeniden yüklenir.
- Executor için `--override-scene` parametresi

- `ButtonContent` iyileştirmeleri - artık `ButtonMessage::Content` kullanarak düğmenin metin alanını yeniden oluşturabilirsiniz

- Eklentiler için kontrol akışı anahtarına erişim sağlar.

- `Plugin::on_ui_message`

- İki adımlı eklenti başlatma:

- `PluginConstructor` özelliği, `Plugin` özelliğinin bir örneğini oluşturan bir yöntem tanımlar, eklenti örneği

oluşturucu, isteğe bağlı olarak eklentiler oluşturmak için kullanılır. Motor, eklenti başlatmayı ertelediği için bu gereklidir.

- `Framework` kaldırıldı, işlevselliği eklentilerle birleştirildi.

- `ScriptConstructorContainer::add` tanımı basitleştirildi, sadece görsel karmaşaya neden olan gereksiz genel parametreler vardı

.

- `NavmeshAgent` için `Clone+Debug` özellikleri uygulandı

- Herhangi bir dosya değiştirildiğinde editördeki günlüğünde spam sorunu düzeltildi.

- Editör için yüksek DPI ekran desteği.

- Editörde yeni oluşturulan kameralar artık varsayılan olarak etkindir.

- Dünya görüntüleyicide kameralar için “Önizleme” seçeneği eklendi.

- Yeniden düzenlenmiş eklemler:

- Eklem bağlama artık tamamen otomatiktir ve eklemin dünya dönüşümüne dayanır, yerel çerçeveleri manuel olarak

  - Bir eklem konumunu değiştirdiğinde yeniden bağlama gerçekleşir

  - Editörde eklem düzenleme artık çok daha sezgiseldir

- Fizik için hata ayıklama görselleştirmesi iyileştirildi.

- NumericUpDown ve Vec2/Vec3/Vec4 widget'ları için salt okunur mod
- Sahne önizleyicide mevcut seçimin global koordinatlarını göster

- BitField widget'ı - sayıları bit konteynerleri olarak düzenlemenize yardımcı olur, ayrı bitler arasında geçiş yapmanızı sağlar

- Inspector'da özellikler için daha kompakt editörler

- NumericUpDown widget'ı artık varsayılan olarak kelime sarma özelliğini kullanmaz

- CheckBox widget'ı artık sadece sol fare tuşuyla değiştirilebilir

- Bir eklemin bağlı gövdeleri arasındaki temasları devre dışı bırakma özelliği

- Proje şablonu oluşturucu için `style` parametresi - varsayılan olarak hangi sahnenin kullanılacağını tanımlar - `2d`

veya `3d`

- `Rectangle` düğümlerinde render edilecek doku kısmını seçme özelliği.

- Şablon oluşturucu için komut dosyası iskeleti oluşturma özelliği

- HSL renk modeli

- Günlük girişlerini panoya kopyalama özelliği

- `Log` API iyileştirmeleri

- Editörde kameraları görselleştirme

- Varlık öğeleri için bağlam menüsü, artık öğeleri açma, silme, gezginde gösterme ve ayrıca

dosya adını ve tam dosya yolunu panoya kopyalama

- Editörde nokta ve spot ışıkları görselleştirme.

# 0.26

- Executor için `--override-scene` parametresi

- `ButtonContent` iyileştirmeleri - artık `ButtonMessage::Content` kullanarak düğmenin metin alanını yeniden oluşturabilirsiniz

- Eklentiler için kontrol akışı anahtarına erişim sağlar.

- `Plugin::on_ui_message`

- İki adımlı eklenti başlatma:

- `PluginConstructor` özelliği, `Plugin` özelliğinin bir örneğini oluşturan bir yöntem tanımlar, eklenti örneği

oluşturucu, isteğe bağlı olarak eklentiler oluşturmak için kullanılır. Motor, eklenti başlatmayı ertelediği için bu gereklidir.

- `Framework` kaldırıldı, işlevselliği eklentilerle birleştirildi.

- `ScriptConstructorContainer::add` tanımı basitleştirildi, sadece görsel karmaşaya neden olan gereksiz genel parametreler vardı

.

- `NavmeshAgent` için `Clone+Debug` özellikleri uygulandı

- Herhangi bir dosya değiştirildiğinde editördeki günlüğünde spam sorunu düzeltildi.

- Editör için yüksek DPI ekran desteği.

- Editörde yeni oluşturulan kameralar artık varsayılan olarak etkindir.

- Dünya görüntüleyicide kameralar için “Önizleme” seçeneği eklendi.

- Yeniden düzenlenmiş eklemler:

- Eklem bağlama artık tamamen otomatiktir ve eklemin dünya dönüşümüne dayanır, yerel çerçeveleri manuel olarak

  - Bir eklem konumunu değiştirdiğinde yeniden bağlama gerçekleşir

  - Editörde eklem düzenleme artık çok daha sezgiseldir

- Fizik için hata ayıklama görselleştirmesi iyileştirildi.

- NumericUpDown ve Vec2/Vec3/Vec4 widget'ları için salt okunur mod
- Sahne önizleyicide mevcut seçimin global koordinatlarını göster

- BitField widget'ı - sayıları bit konteynerleri olarak düzenlemenize yardımcı olur, ayrı bitler arasında geçiş yapmanızı sağlar

- Inspector'da özellikler için daha kompakt editörler

- NumericUpDown widget'ı artık varsayılan olarak kelime sarma özelliğini kullanmaz

- CheckBox widget'ı artık sadece sol fare tuşuyla değiştirilebilir

- Bir eklemin bağlı gövdeleri arasındaki temasları devre dışı bırakma özelliği

- Proje şablonu oluşturucu için `style` parametresi - varsayılan olarak hangi sahnenin kullanılacağını tanımlar - `2d`

veya `3d`

- `Rectangle` düğümlerinde render edilecek doku kısmını seçme özelliği.

- Şablon oluşturucu için komut dosyası iskeleti oluşturma özelliği

- HSL renk modeli

- Günlük girişlerini panoya kopyalama özelliği

- `Log` API iyileştirmeleri

- Editörde kameraları görselleştirme

- Varlık öğeleri için bağlam menüsü, artık öğeleri açma, silme, gezginde gösterme ve ayrıca

dosya adını ve tam dosya yolunu panoya kopyalama

- Editörde nokta ve spot ışıkları görselleştirme.

## Migration guide

Bu sürümde önemli değişiklikler yoktur.

# 0.25

- Statik eklenti sistemi

- Kullanıcı tanımlı komut dosyaları

- Editör için oynatma modu

- Animasyon Karıştırma Durum Makinesi (ABSM) editörü.

- Bazı ses öğeleri sahne grafiğine entegre edildi.

- Yeni `Sound` ve `Listener` sahne düğümleri.

- Ses arabelleği içe aktarma seçenekleri.

- `ResourceManager::request_sound_buffer` artık yalnızca ses arabelleğine giden yolu kabul ediyor.

- Prefab miras iyileştirmeleri - artık sahne düğümlerinin çoğu miras alınabilir.

- Fiziğin simülasyon özelliklerine erişim.

- Motor ve Kaynak yöneticisi artık serileştirilemez, doğru şekilde

kaydetme dosyalarını oluşturmak için geçiş kılavuzuna bakın.

- `Node` numaralandırması kaldırıldı ve dinamik gönderme ile değiştirildi. Bu, kendi

sahne düğüm türlerinizi tanımlamanıza olanak tanır.

- `Base` artık bir sahne düğümü değildir, `Pivot` düğümü ile değiştirilmiştir (daha fazla bilgi için geçiş kılavuzuna bakın)

- `Base` artık `cast_shadows` özelliğine sahiptir, ilgili özellik ayarlayıcıları/alıcıları `Mesh` ve

`Terrain` düğümlerinden kaldırılmıştır.

- ListView öğesini görüntüye getirme özelliği.

- Günlük kaydedici iyileştirmeleri: olay abonelikleri + zaman damgalarının toplanması

- Editörde günlük paneli iyileştirmeleri: önem derecesi filtreleme, renk ayrımı.

- Sahne düğümleri artık daha doğru yerel sınırlara sahiptir (düğüme sığabilen bir sınırlayıcı kutu).

- Editörde seçim iyileştirmeleri: artık düğümün geometrisine karşı hassas vuruş testi kullanılıyor.

- Editörde seçim için “Arka yüzleri yok say” seçeneği: poligon yüzlerinin “arkasından” seçim yapmanızı sağlar,

özellikle kapalı ortamlar için kullanışlıdır.

- Döndürme şeritleri torus ile değiştirildi, istenen döndürme modunu seçmek çok daha kolay.

- Derinlik sorunlarını önleyen, düzenleyicide gizmo'lar için yeni malzeme.

- TreeView widget'ı için yeni genişletici, `+` ve `-` işaretleri yerine `V` ve `>` okları.

- ScrollBar widget'ı varsayılan olarak çok daha ince.

- Editör ayarları penceresi artık veri görselleştirme için tek tip bir yol sağlayan Inspector widget'ına dayanmaktadır.

- `DEFAULT_FONT` singleton kaldırıldı, yerine `default_font`

- Editörde kısayollar iyileştirildi.

- Genel UI performansı iyileştirildi.

- Widget sınırlarının üst öğe sınırlarına kırpılmasını devre dışı bırakma özelliği.
- Widget'lar için düzen ve görüntüleme dönüştürme desteği - widget'ları ölçeklendirmenize/döndürmenize/yer değiştirmenize olanak tanır.

- Widget'ı hiyerarşide en altta yapma özelliği.

- Animasyon karıştırma durum makinesi yeniden düzenleme, optimizasyonlar ve kararlılık iyileştirmeleri.

- Animasyon karıştırma durum makineleri artık Scene'de depolanan özel bir kapta saklanıyor.

- Yerleştirme yöneticisi artık yalnızca kendi pencereleri için bağlantı noktalarını gösterir.

- Model önizleyicisi artık çok daha sezgisel denetimlere sahiptir.

- NumericUpDown artık sayısal sınırların kenarlarında paniğe kapılmıyor (örneğin, `i32::MAX_VALUE + 1` yapmaya çalışırken).

- UI için çift tıklama desteği.

- Editör için güncelleme hızı düzeltildi, metin kutularındaki titreme sorununu giderir.

- Mevcut seçim kısıtlama yığınıyla sınırlı olmayan vuruş testi gerçekleştiren `UserInterface::hit_test_unrestricted`.

- WASM renderer düzeltmeleri.
- Geçersiz tanıtıcılarda paniğe kapılmak yerine `Option<T>` döndüren `Pool::try_free`.

- Model önizleyici için ışık kaynağı.

- Model önizleyici için ışık kaynağı

- Editör ve model önizleyici kameraları için varsayılan skybox

- `Color` API iyileştirmeleri.

- `#[reflect(expand)]` ve `#[reflect(expand_subtree)]`, `Inspect` proc-macro'dan kaldırıldı

- Enum varyantları için doğru alan adı oluşturma

- UI'da Bézier eğrileri çizme yeteneği.

- Çok katmanlı navigasyon ağlarının navmesh ajanı navigasyonu için düzeltme.

- Serileştirici için iyileştirmeler, artık serileştirme hatalarından doğru şekilde kurtulabilirsiniz.

## Migration guide

**UYARI:** Bu sürüm, eski ses sistemini yeni sisteme dönüştürmez, yani

sahnenizde herhangi bir ses varsa, bunlar kaybolacaktır!
Artık `fyrox_sound` varlıklarına sınırlı erişim vardır, ses bağlamları, sesler ve

efektler manuel olarak oluşturulamaz. İlgili sahne düğümlerini (`Sound`, `Listener`) ve `fyrox::scene::sound` modülünden (ve alt modüllerinden) `Effect` kullanmanız gerekir.

### Düğümler

`Node` numaralandırması kaldırıldığından, düğümleri yönetmenin yeni bir yolu vardır:

- `Node` artık yeni bir tip yapısında sarılmış `Box<dyn NodeTrait>` şeklindedir.

- Desen eşleştirme, `cast` ve `cast_mut` yöntemleriyle değiştirilmiştir.

- `cast/cast_mut`'a ek olarak, polimorfizm için iki daha karmaşık yöntem vardır: `query_component_ref` ve

`query_component_mut`, düğümlerin iç kısımlarına referansları çıkarabilir. Bunun artık tek bir

kullanımı vardır - `Light` sıralaması kaldırıldı ve `PointLight`, `SpotLight`, `DirectionalLight`, `query_component_ref/query_component_mut` aracılığıyla
`query_component_ref/query_component_mut` aracılığıyla `BaseLight` bileşenine. `query_component`, bileşeni sorgulamaya çalışırken ek dallanma gerektirebileceğinden

biraz daha yavaş olabilir.

- `Base` düğümü, `Pivot` düğümüyle (ve ilgili `PivotBuilder` ile) değiştirildi. Bu,

`Deref<Target = Base>/DerefMut` uygulamasındaki sorunlar nedeniyle gerçekleşti. `Base`, `NodeTrait` uyguluyorsa, `Deref`'i de uygulamalıdır,

ancak `Base` için `Deref`'i uygulamak sonsuz deref zorlama döngüsüne neden olur.

- Özel sahne düğümleri oluşturabilmek ve bu düğümlerle sahne grafiğini serileştirebilmek/serileştirmeyi kaldırabilmek için

`NodeConstructorContainer` eklendi. Bu, serileştirmeyi kaldırma aşamasında uuid türüne göre doğru düğüm oluşturucuyu seçmeyi sağlayan basit bir `UUID -> NodeConstructor` haritası içerir.

#### `BaseBuilder`'ı `PivotBuilder` ile değiştirme

Çok basit, sadece `BaseBuilder`'ı bir `PivotBuilder` ile sarın ve `PivotBuilder` örneğinde `build`'u çağırın:

```rust
// Before
fn create_pivot_node(graph: &mut Graph) -> Handle<Node> {
    BaseBuilder::new().build(graph)
}

// After
fn create_pivot_node(graph: &mut Graph) -> Handle<Node> {
    PivotBuilder::new(BaseBuilder::new()).build(graph)
}
```

#### Desen eşleştirme ile değiştirme

Desen eşleştirme, 4 yeni yöntemle değiştirildi. `cast/cast_mut/query_component_ref/query_component_mut`:

```rust
fn set_sprite_color(node: &mut Node, color: Color) {
    // Gerçek düğüm türünden emin olduğunuzda `cast_mut` kullanın.
    if let Some(sprite) = node.cast_mut::<Sprite>() {
        sprite.set_color(color);
    }
}

fn set_light_color(node: &mut Node, color: Color) {
    // Düğümün tam türünün ne olduğundan emin değilseniz query_component_mut kullanın.


    // Bu örnekte `node`, PointLight, SpotLight veya DirectionalLight olabilir,

// çünkü hepsi `query_component_x` aracılığıyla `BaseLight`'a erişim sağlar, bu nedenle işlev

// bu türlerin herhangi biriyle çalışacaktır.
    if let Some(base_light) = node.query_component_mut::<BaseLight>() {
        base_light.set_color(color);
    }
}
```

### Dinleyici

Artık ses dinleyicinin konumunu ve yönünü manuel olarak senkronize etmenize gerek yok, tek yapmanız gereken

`Listener` düğümü oluşturmak ve bunu ana kameranıza (veya başka bir sahne düğümüne) eklemek.

Motorun yalnızca bir dinleyiciyi desteklediğini, yani aynı anda yalnızca bir dinleyicinin aktif olabileceğini

unutmayın. Motor, birden fazla dinleyicinin aktif olmasını engellemez, ancak ses çıkışı için yalnızca ilki (

sırası tanımlanmamıştır) kullanılır.

### Ses kaynakları

Sesler arasında artık 2D/3D ayrımı yoktur, tüm sesler varsayılan olarak 3D'dir. Artık her ses kaynağı

bir sahne düğümüdür ve şu şekilde oluşturulabilir:

```rust
let sound = SoundBuilder::new(
BaseBuilder::new().with_local_transform(
TransformBuilder::new()
.with_local_position(position)
.build(),
),
)
.with_buffer(buffer.into())
.with_status(Status::Playing)
.with_play_once(true)
.with_gain(gain)
.with_radius(radius)
.with_rolloff_factor(rolloff_factor)
.build(graph);
```

API'si `fyrox_sound` API'sini taklit eder, bu nedenle geçişte sorun yaşanmamalıdır.

### Efektler

Efektler, `Sound` sahne düğümleriyle uyumlu hale getirmek için `fyrox_sound` etrafında ince bir sarmalayıcıya sahiptir. Bir reverb

efekt örneği şu şekilde oluşturulabilir:

```rust
let reverb = ReverbEffectBuilder::new(BaseEffectBuilder::new().with_gain(0.7))
.with_wet(0.5)
.with_dry(0.5)
.with_decay_time(3.0)
.build( & mut scene.graph.sound_context);
```

Bir ses kaynağı, bir efektin üzerine şu şekilde eklenebilir:

```rust
graph
.sound_context
.effect_mut( self .reverb)
.inputs_mut()
.push(EffectInput {
sound,
filter: None,
});
```

### Filtreler

Etki girişi filtreleri API'si değişmeden kalmıştır.

### Motor başlatma

`Engine::new` imzası, `EngineInitParams`'ı kabul edecek şekilde değiştirilmiştir, önceki tüm argümanlar

yapısına taşınmıştır. Ancak, `serialization_context` ve

`resource_manager` gibi bazı yeni motor başlatma parametreleri eklenmiştir. Önceden `resource_manager` örtük olarak oluşturulurken, artık

dışarıda oluşturulup `EngineInitParams`'a aktarılması gerekmektedir. Bunun nedeni, motor içinde kullanılabilecek ve harici eklentiler tarafından eklenebilecek çeşitli türler için bir dizi yapıcı içeren yeni `SerializationContext`'tir

.

Tipik bir motor başlatma işlemi aşağıdaki gibi görünebilir:

```rust
use fyrox::engine::{Engine, EngineInitParams};
use fyrox::window::WindowBuilder;
use fyrox::engine::resource_manager::ResourceManager;
use fyrox::event_loop::EventLoop;
use std::sync::Arc;
use fyrox::engine::SerializationContext;

fn init_engine() {
    let evt = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Test")
        .with_fullscreen(None);
    let serialization_context = Arc::new(SerializationContext::new());
    let mut engine = Engine::new(EngineInitParams {
        window_builder,
        resource_manager: ResourceManager::new(serialization_context.clone()),
        serialization_context,
        events_loop: &evt,
        vsync: false,
    })
        .unwrap();
}
```

## Seri hale getirme

Engine ve ResourceManager artık seri hale getirilemez. Bu, oyunlarda kayıt dosyalarının oluşturulma yaklaşımını değiştirir.

Önceden şunun gibi bir şey kullanıyordunuz (aşağıdaki kod parçacıkları `save_load` örneğinin değiştirilmiş versiyonlarıdır):

```rust
const SAVE_FILE: &str = "save.bin";

fn save(game: &mut Game) {
    let mut visitor = Visitor::new();

    game.engine.visit("Engine", visitor)?; // This no longer works
    game.game_scene.visit("GameScene", visitor)?;

    visitor.save_binary(Path::new(SAVE_FILE)).unwrap();
}

fn load(game: &mut Game) {
    if Path::new(SAVE_FILE).exists() {
        if let Some(game_scene) = game.game_scene.take() {
            game.engine.scenes.remove(game_scene.scene);
        }

        let mut visitor = block_on(Visitor::load_binary(SAVE_FILE)).unwrap();

        game.engine.visit("Engine", visitor)?; // This no longer works
        game.game_scene.visit("GameScene", visitor)?;
    }
}
```

Ancak, uygulamada bu yaklaşım bazı istenmeyen yan etkilere yol açabilir. Eski yaklaşımın temel sorunu,

motoru serileştirdiğinizde, sahip olduğunuz tüm sahneleri serileştirmesidir. Tek bir sahneniz varsa bu durum

sorun teşkil etmez, ancak iki veya daha fazla sahneniz varsa (örneğin, biri menü ve diğeri

oyun seviyesi için), gereksiz veriler yazılır/okunur. İkinci sorun, eski yaklaşımı kullanarak kaydedilmiş oyunları asenkron olarak yükleyememenizdir

çünkü bu, motorun değiştirilebilir erişimini gerektirir ve işlerin iş parçacığı dışında çalışmasını engeller.

Yeni yaklaşım çok daha esnektir ve bu tür sorunlar yoktur. Motorun tüm durumunu kaydetmek yerine,

yalnızca gerçekten ihtiyacınız olanları kaydedip yükleyebilirsiniz:

```rust
const SAVE_FILE: &str = "save.bin";

fn save(game: &mut Game) {
    if let Some(game_scene) = game.game_scene.as_mut() {
        let mut visitor = Visitor::new();

        // Serialize game scene first.
        game.engine.scenes[game_scene.scene]
            .save("Scene", &mut visitor)
            .unwrap();
        // Then serialize the game scene.
        game_scene.visit("GameScene", &mut visitor).unwrap();

        // And call save method to write everything to disk.
        visitor.save_binary(Path::new(SAVE_FILE)).unwrap();
    }
}

// Notice that load is now async.
async fn load(game: &mut Game) {
    // Try to load saved game.
    if Path::new(SAVE_FILE).exists() {
        // Remove current scene first.
        if let Some(game_scene) = game.game_scene.take() {
            game.engine.scenes.remove(game_scene.scene);
        }

        let mut visitor = Visitor::load_binary(SAVE_FILE).await.unwrap();

        let scene = SceneLoader::load("Scene", &mut visitor)
            .unwrap()
            .finish(game.engine.resource_manager.clone())
            .await;

        let mut game_scene = GameScene::default();
        game_scene.visit("GameScene", &mut visitor).unwrap();

        game_scene.scene = game.engine.scenes.add(scene);
        game.game_scene = Some(game_scene);
    }
}
```

Yeni yaklaşımda gördüğünüz gibi, sahnenizi ve bazı seviye verilerini kaydediyorsunuz ve yükleme sırasında sahneyi yüklüyor,

her zamanki gibi motora ekliyor ve seviye verilerini yüklüyorsunuz. Yeni yaklaşım biraz daha ayrıntılıdır, ancak çok daha

esnektir.

# 0.24

## Motor

- 2D oyun desteği (2D fizik ile birlikte)

- Üç yeni sahne düğümü eklendi: RigidBody, Collider, Joint. Rigid body, collider ve joint artık grafik düğümleri olduğundan,

bunlarla karmaşık hiyerarşiler oluşturmak mümkün.

- Katı cisimleri sahne grafiğindeki herhangi bir düğüme eklemek mümkündür, bu durumda konumu doğru olacaktır (

önceden katı cisimleri yalnızca kök sahne düğümlerine eklemek mümkündü).

- Yeni `Inspector` widget'ı + çok sayıda yerleşik özellik düzenleyici (özel düzenleyiciler ekleme özelliği ile)

- Hafif yansıma için `Inspect` özelliği + proc makrosu

- UI artık dinamik gönderme kullanıyor, böylece özel düğümler ve mesajlar kolayca eklenebilir

- fyrox-sound optimizasyonları (30% daha hızlı)

- Örnekleme oranı != 1.0 olduğunda ses örnekleri için doğrusal enterpolasyon (öncekinden çok daha iyi kalite)

- Malzeme düzenleyicideki renk alanları artık düzenlenebilir

- Pencere istemci alanı artık sadece Windows'ta değil, tüm işletim sistemlerinde renderer tarafından doğru şekilde dolduruluyor.

- NumericRange kaldırıldı (standart Range + extension özelliği ile değiştirildi)

- FileBrowser/FileSelector widget'larında dosya ve dizinleri sıralama

- Ses için RawStreaming veri kaynağı

- Renderer performans iyileştirmeleri (2,5 kat daha hızlı)

- UI düzeni performans iyileştirmeleri

- Renderer'ın gigabaytlarca RAM tüketmesini önleme

- Çapraz kutu içi yerleştirmeyi etkinleştirmek için `#[inline]` özniteliğini kullanma

- Statik dizelerin daha hızlı karma hale getirilmesi için `ImmutableString`

- `Pool` için hafif bir analog olarak `SparseBuffer` (nesil olmayan sürüm)

- FBX malzemelerinde diffüz renk desteği

- Arazide frustum culling düzeltmeleri

- Derleme başarılı olduğunda gölgelendiriciler boş satır yazdırmaz.

- `Pool` iyileştirmeleri

- `Pool` referansları için `IntoIterator` impl

- Yönlü ışık kaynakları için basamaklı gölge haritaları

- `Pool` için `spawn_at` + `spawn_at_handle`

- Sürükle ve bırak için önizleme

- `Grid` widget düzeni performans optimizasyonları (**1000x** performans artışı - bu bir yazım hatası değildir)

- UI widget'ları için `query_component`

- Eğri kaynağı

- Widget'ı silerken ilgili tüm widget'ları kaldır (sarkan nesneler bırakma)

- Dünya sınırlayıcı kutu hesaplama düzeltmesi

- UI rutinlerinde geçersiz kılma işleminin yoğun kullanımı (her karede çok sayıda widget'ın kontrol edilmesini önler)

- `parking-lot` senkronizasyon ilkellerine geçiş

- `FxHash`'e geçiş (daha hızlı karma)

- `Result<(), Error` hatalarını günlüğe kaydetmek için `Log::verify`

- Özel sahne düğümü özellikleri desteği

- `Alt+Tıklama`, `tree` widget'ında seçimi engeller

- Kamera projeksiyonunu değiştirme yeteneği (Perspektif veya Ortografik)

- Açılır pencereler için akıllı konum seçimi (ekran sınırlarının dışında görünmelerini engeller)

- Lanczos filtresi kullanılarak yüksek kaliteli mip-map oluşturma.

## Editor

- `Inspector` widget entegrasyonu, tonlarca boilerplate kodunu kaldırmaya olanak sağladı

- Orta fare tuşuyla kamera sürükleme

- Q/E + Boşluk tuşuyla kamerayı yukarı/aşağı hareket ettirme

- Çalışma dizini mesajı artık çok daha az kafa karıştırıcı
- Editörde ses kaynaklarını düzenleme özelliği

- Dünya görüntüleyicide dama tahtası renklendirme düzeltmesi

- Dünya görüntüleyicide arama

- Arazi editörü için yüzen fırça paneli

- Editör kamerasında manuel pozlama (otomatik pozlamadan etkilenmez)

- Eğri düzenleyici

- Sahne düğümünün yeni oluşturulan örneğini otomatik olarak seçme

- grid yapıştırma düzeltmesi

- Açı yapıştırma

- Birden fazla seçili nesnenin özelliklerini aynı anda düzenleme.

- Dünya görüntüleyicide sahne öğeleri için bağlam menüsü

- Sahne öğesi bağlam menüsü için `Create sub-item`

- Varlık tarayıcısı için içe aktarma seçenekleri düzenleyici

- Dokular için hızlı yeniden yükleme.

## Önemli değişiklikler ve geçiş kılavuzu

Bu sürümde birçok önemli değişiklik vardır, ancak bunların çoğu kodla ilgilidir ve

önceki sürümde oluşturulan sahneler hala yüklenebilir olmalıdır.

### Eski sahneleri yeni biçime dönüştürün

İlk olarak, crates.io'dan rusty-editor'ü yükleyin ve çalıştırın:

```shell
cargo install rusty-editor
rusty-editor
```

Ardından sahnelerinizi tek tek yeniden kaydedin. Bu işlemden sonra tüm sahneleriniz en yeni sürüme dönüştürülecektir.

GitHub deposundaki düzenleyicinin (0.25+) artık geriye dönük uyumluluk/dönüştürme koduna sahip olmadığını unutmayın!

### 2D sahne

2D sahneler tamamen kaldırıldı ve neredeyse tüm 2D düğümleri kaldırıldı, geriye sadece bir “2D” düğümü kaldı - Rectangle.

2D artık 3D sahnelerde uygulanıyor, bunun için ortografik kamera kullanmanız gerekiyor. 2D sahneler için geçiş kılavuzu yoktur

çünkü 2D temel düzeyde destekleniyordu ve motorun 2D'sini kullanan herhangi bir proje olduğunu sanmıyorum.

## Kaynak yönetimi

Kaynak yöneticisi API'sini değiştirdi ve size zaman kazandıracak bazı kullanışlı özellikler kazandı.

`request_texture` artık yalnızca bir argüman kabul ediyor: doku yolu. İkinci argüman

`TextureImportOptions` aktarmak için kullanılıyordu. İçe aktarma seçenekleri artık ayrı bir seçenek dosyasında bulunmalıdır. Örneğin,

`foo.jpg` adlı bir dokumanız var ve bunun içe aktarma seçeneklerini (sıkıştırma, sarma modları, mip haritaları vb.) değiştirmek istiyorsunuz. Bunu yapmak için,

dosyanızın bulunduğu dizinde aşağıdaki içeriğe sahip `foo.jpg.options` dosyasını oluşturmalısınız (her alan

isteğe bağlıdır):

```text
(
    minification_filter: LinearMipMapLinear,
    magnification_filter: Linear,
    s_wrap_mode: Repeat,
    t_wrap_mode: Repeat,
    anisotropy: 16,
    compression: NoCompression,
)
```

Motor, `request_texture` çağrıldığında bu dosyayı okuyacak ve ilk yüklemede seçenekleri uygulayacaktır.

Bu dosya zorunlu değildir, her zaman kaynak yöneticisinde

`set_texture_import_options`

`request_model` de aynı değişiklikler yapılmıştır, tek bir argüman vardır ve içe aktarma seçenekleri seçenekler dosyasına taşınmıştır:

```text
(
    material_search_options: RecursiveUp
)
```

Yine, tüm alanlar zorunlu değildir ve dosyanın tamamı atlanabilir, global import varsayılanları

`set_model_import_options`

### Fizik

Eski fizik, yeni sahne düğümleriyle değiştirildi: RigidBody, Collider, Joint. Eski sahneler yüklendiğinde otomatik olarak dönüştürülecektir,

sahnelerinizi mümkün olan en kısa sürede editör kullanarak dönüştürmelisiniz (sahnenizi açın ve kaydedin, bu

dönüştürmeyi gerçekleştirecektir).

Artık bir sahne düğümüne rijit bir cisim eklemenin iki yolu vardır:

- Nesnenizin rijit bir cisme sahip olmasını istiyorsanız (örneğin, kutu rijit cisimli bir sandık), nesneniz

rijit bir cismin **alt** nesnesi olmalıdır. Grafiksel olarak şöyle gösterilebilir:

```text
- Rigid Body
  - Crate3DModel
  - Cuboid Collider
```

- Nesnenizin, nesnenizle birlikte hareket etmesi gereken sert bir gövdeye sahip olmasını istiyorsanız (örneğin, çarpma kutularını simüle etmek için),

sert gövde nesnenizin alt nesnesi olmalıdır. Ayrıca, `Kinematic` olarak işaretlenmelidir,

aksi takdirde simülasyondan etkilenecektir (basitçe söylemek gerekirse, yere düşecektir). Grafiksel olarak

şu şekilde gösterilebilir:

```text
- Limb
  - Rigid Body
     - Capsule Collider
```

#### Migration

Bu bölüm, yeni fizik sistemine geçiş yapmanıza yardımcı olacaktır.

##### Rigid bodies

Rigi body ve collider artık şu şekilde oluşturulabilir:

(bazı yerlede rigi body yerine sert cisimler collider yerinede çarpışanlar görebilirsiniz bu çeviriden kaynaklıdır.)

```rust
use fyrox_impl::{
    core::{algebra::Vector3, pool::Handle},
    scene::{
        base::BaseBuilder,
        collider::{ColliderBuilder, ColliderShape},
        node::Node,
        rigidbody::RigidBodyBuilder,
        transform::TransformBuilder,
        Scene,
    },
};

fn create_capsule_rigid_body(scene: &mut Scene) -> Handle<Node> {
    RigidBodyBuilder::new(
        BaseBuilder::new()
            .with_local_transform(
                // Sert gövdeyi konumlandırmak ve döndürmek için motorun dönüşümünü kullanmalısınız.
                TransformBuilder::new()
                    .with_local_position(Vector3::new(1.0, 2.0, 3.0))
                    .build(),
            )
            .with_children(&[
                // En az bir alt çarpışma düğümü eklemek çok önemlidir, aksi takdirde katı

                // gövde çarpışma tepkisi vermez.
                ColliderBuilder::new(
                    BaseBuilder::new().with_local_transform(
                        // Çarpışanlar, ana katı cisimlerine göre göreceli konuma sahip olabilir.
                        TransformBuilder::new()
                            .with_local_position(Vector3::new(0.0, 0.5, 0.0))
                            .build(),
                    ),
                )
                    // Diğer özellikler neredeyse önceki gibi ayarlanabilir.
                    .with_friction(0.2)
                    .with_restitution(0.1)
                    .with_shape(ColliderShape::capsule_y(0.5, 0.2))
                    .build(&mut scene.graph),
            ]),
    )
        // Rest of properties can be set almost as before.
        .with_mass(2.0)
        .with_ang_damping(0.1)
        .with_lin_vel(Vector3::new(2.0, 1.0, 3.0))
        .with_ang_vel(Vector3::new(0.1, 0.1, 0.1))
        .build(&mut scene.graph)
}
```

##### Joints

joints de benzer şekilde oluşturulabilir:

```rust
fn create_ball_joint(scene: &mut Scene) -> Handle<Node> {
    JointBuilder::new(BaseBuilder::new())
        .with_params(JointParams::BallJoint(BallJoint {
            local_anchor1: Vector3::new(1.0, 0.0, 0.0),
            local_anchor2: Vector3::new(-1.0, 0.0, 0.0),
            limits_local_axis1: Vector3::new(1.0, 0.0, 0.0),
            limits_local_axis2: Vector3::new(1.0, 0.0, 0.0),
            limits_enabled: true,
            limits_angle: 45.0,
        }))
        .with_body1(create_capsule_rigid_body(scene))
        .with_body2(create_capsule_rigid_body(scene))
        .build(&mut scene.graph)
}
```

##### Raycasting

Raycasting, `scene.graph.physics` içinde bulunur ve neredeyse hiç değişiklik yapılmamıştır, ancak artık ham çarpışan nesne tutamaçları yerine

sahne düğümlerine tutamaçlar döndürür.

##### İletişim bilgileri

İletişim bilgileri artık `contacts()` yöntemi ile çarpıştırıcı düğümünden sorgulanabilir.

```rust
fn query_contacts(collider: Handle<Node>, graph: &Graph) -> impl Iterator<Item=ContactPair> {
    graph[collider].as_collider().contacts(&graph.physics)
}
```
