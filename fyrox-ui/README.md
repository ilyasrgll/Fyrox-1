# fyrox-ui

Tutulan mod, genel amaçlı, grafik API agnostik kullanıcı arayüzü kütüphanesi. WPF'den esinlenilmiştir.

**NOT:** Bu sandığın adında `fyrox` öneki olmasına rağmen, herhangi bir sorun olmadan ayrı olarak kullanılabilir.

## Özellikler

- 28'den fazla widget
- Tam TTF/OTF yazı tipi desteği
- Güçlü yerleşim sistemi
- Tamamen özelleştirilebilir - herhangi bir karmaşıklıkta görsel ağaçlar oluşturabilirsiniz: örneğin bir ağaç görünümü öğesi içerik olarak herhangi bir alt araca sahip olabilir.
- GAPI-agnostik - bu sandık oluşturma arka ucu hakkında hiçbir şey bilmez: OpenGL, DirectX, Vulkan, Metal veya hatta yerleşik işletim sistemi çizim API'si olabilir.
- OS-agnostik - tüm işletim sistemleri ve pencere yöneticilerinde tüm widget'ların benzer görünümü.
- Genişletilebilir - kullanıcı tanımlı widget'lar için tam destek.

## Widget'lar

- [x] Button
- [x] Border
- [x] Canvas
- [x] Color picker
- [x] Color field
- [x] Check box
- [x] Decorator
- [x] Drop-down list
- [x] Grid
- [x] Image
- [x] List view
- [x] Popup
- [x] Progress bar
- [x] Scroll bar
- [x] Scroll panel
- [x] Scroll viewer
- [x] Stack panel
- [x] Tab control
- [x] Text
- [x] Text box
- [x] Tree
- [x] Window
- [x] File browser
- [x] File selector
- [x] Docking manager
- [x] NumericUpDown
- [x] Vector2/Vector3/Vector4 editor
- [x] Quaternion editor
- [x] Menu
- [x] Menu item
- [x] Message box
- [x] Wrap panel
- [x] Curve editor
- [x] Bit Field
- [x] User defined widget
- [x] Inspector

## Sınırlamalar

- Bu kütüphane OS-, GAPI-agnostik olduğundan, yerel OS pencereleri oluşturamaz ve ekranda herhangi bir şey oluşturamaz. Bunun yerine, oyununuzda/uygulamanızda yorumlanması gereken komutların bir listesini tutan dahili bir çizim tamponu kullanır. Bu çok esnek bir yoldur, ancak bazı sınırlamaları vardır: çoklu pencere (yerel) yapılandırmasının uygulanması zordur, kendi kullanıcı arayüzü oluşturucunuzu uygulamanız gerekir, bu da böyle bir şeye aşina değilseniz zor olabilir.
- Hala klavye navigasyonu yok, planlanıyor ancak yüksek öncelikli değil.
- Sağdan sola metin desteği yok (Arapça, İbranice, vb.)

## Performans

- Genel olarak fyrox-ui hızlıdır, ancak yanlış kullanılırsa yavaş olabilir. Bu kütüphane çok karmaşık bir düzen sistemi kullandığından, çok sayıda ui öğesi hareket ettiriliyorsa (örneğin kaydırma sırasında) yavaş çalışabilir. Umarım yerleşik düzen önbellekleme sistemine sahiptir ve düzen geçersiz kılmaya dayanır, bu nedenle her karede düzen hesaplamaları yapmaz - yalnızca önemli bir şey değiştiğinde (konum, boyut vb.).
- Render komutlarının gruplanması zor olabilir, çünkü bu kütüphane yaygın olarak kırpma kullanır ve her kırpma geometrisi ayrı bir draw çağrısı olarak stencil tamponuna çizilmelidir. Render hala optimize edilmelidir, şimdilik verimsizdir.

## Şekillendirme

fyrox-ui biraz alışılmadık bir şekillendirme yöntemi kullanır - widget'ın görsel ağaçlarının tüm alt grafiklerini değiştirmeniz gerekir. Bu ne anlama geliyor? fyrox-ui herhangi bir karmaşıklıkta görsel ağaçlar oluşturmak için grafik kullanır, her widget grafikteki bir düğüm kümesidir. Örneğin bir düğme, arka plan ve ön plan widget'larından oluşan bir kümedir, arka plan widget'ı genellikle görünümü tanımlar ve ön plan bir içeriği gösterir. Bir düğmenin içeriği herhangi bir widget olabilir, en yaygın durumlarda ya bir metin ya da bir resimdir. Bu nedenle, bir düğmenin görünümünü değiştirmek için oluşturma aşamasında kendi arka plan widget'ınızı tanımlamanız gerekir, varsayılan olarak fyrox-ui, MouseEnter, MouseLeave vb. mesajı aldığında sadece ön plan fırçasını değiştiren Decorator widget'ını kullanır. Bu gerçek, **küçük** stilleri (renk değiştirmek gibi) önemli ölçüde karmaşıklaştırır, ancak süper esnek bir yaklaşımdır ve kendi benzersiz stilinizi oluşturmanıza olanak tanır. Widget oluşturucuların çoğu parçalarını değiştirmek için bir yol sağlar, bazıları hala böyle bir işlevsellikten yoksun olabilir, ancak bu eninde sonunda düzeltilmelidir.

## Ekran Görüntüleri

![editor](https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/pics/editor.png)
![absm editor](https://fyrox.rs/assets/absm_editor_full.png)
![sound](https://fyrox.rs/assets/reverb_properties.png)

## Katkıda Bulunmak

- Bir kullanıcı arayüzü kütüphanesi yazmak tek bir kişi için çok zordur, bu nedenle her türlü yardım takdir edilmektedir.

## Dokümantasyon

TODO.

## Örnekler

TODO.

Bu UI kütüphanesini kullanan iki proje var:

- [Fyroxed](https://github.com/FyroxEngine/Fyrox/)
- [rusty-shooter](https://github.com/mrDIMAS/rusty-shooter)

Ancak, bu projelerden kütüphanenin nasıl kullanılacağını anlamak çok zor olabilir, bu nedenle bağımsız örnekler eklenmelidir. Bu hala bir TODO'dur.
