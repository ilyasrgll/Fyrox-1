# Katkıda bulunanlar yönergeleri

Bu belge, motora nasıl katkıda bulunabileceğinizi açıklamaktadır. Bunun “uymak zorunda olduğunuz” bir liste olmadığını unutmayın; motora katkıda bulunurken sağduyunuzu kullanın.

## Nasıl katkıda bulunabilirsiniz

- **Kod** — Kod yazmak, katkı sağlamanın en belirgin yoludur. Eksik özellikleri ekleyebilir, hataları düzeltebilir, iyileştirebilir
  mevcut araçlar, vb. daha fazla bilgi için [kod bölümüne](#contributing-code) bakınız.
- **Dokümantasyon** — dokümantasyon katkıda bulunabileceğiniz bir sonraki yerdir, bu motor zaten oldukça büyüktür ve
  bir sürü belgelenmemiş kod var. Belgelenmemiş bir API'ye aşina iseniz, tereddüt etmeyin - yazın
  bunun için dokümantasyon. Bu API'yi kullanacak sonraki insanlar için çok zaman kazandıracaksınız ve onlar
  Belgelerin varlığına minnettar olacaklar.
  Daha fazla bilgi için [dokümantasyon bölümüne](#contributing-documentation) bakınız.
- **Oyun yapmak** — Neyin eksik olduğunu veya geliştirilmesi gerektiğini anlamanın en iyi yolu motoru kullanarak oyun yapmaktır. Eksik veya geliştirilebilir bir şey bulduğunuzda, tereddüt etmeyin - bu depoda bir sorun oluşturun ve belki birileri bu sorunu tespit edip düzeltecektir. Sorunları dosyalamak her zaman iyidir, yanlış bir şey olduğunu açıkça gösterir ve insanlar ilerlemeyi takip edebilir.
- **Hata bildir** — Bir şey olması gerektiği gibi çalışmıyorsa, sorunun açıklığa kavuşması için bu konuda bir sorun dosyası oluşturun. Her yazılımın uç durumları vardır; bunlar, önemsiz olmayan bir şey yapmanız gerekene kadar uzun süre gizlenebilir.
- **Bağış** — kodu veya dokümanlari yazmak i̇çi̇n zamaniniz yoksa ve projeyi̇ canli görmek i̇sti̇yorsaniz, motorun geli̇şti̇ri̇ci̇leri̇ne herhangi̇ bi̇r mi̇ktarda bağişta bulunmayi düşünmeli̇si̇ni̇z.
- **Motoru tanıtın** - yazılar yazmak, videolar hazırlamak, motorla ilgili haberleri sosyal medyada paylaşmak vb.

## Koda katkıda bulunma

Kod katkıları için ortak kurallar:

- **Kodu temiz tutun** — nDeğişkenlerinizi ve fonksiyonlarınızı anlamlı bir şekilde kullanın. Her şeyi tek seferde halleden tanrı benzeri fonksiyonlar yaratmamaya çalışın. Çekme isteği yapmadan önce kodunuzu her zaman derleyin.
- **Belgeleri yazın** — kodunuzu belgeleyin. Kodun yüksek seviyede ne yaptığını açıklamalıdır. Belgelerinize düşük seviyeli ayrıntıları dahil etmeyin (çok önemli bir şeyi açıklamanız gerekmedikçe).
- **Kodunuzu biçimlendirin** — yazdığınız kodu biçimlendirmek için `rustfmt` kullanın.
- **Birim testleri yazın** — motora yeni bir işlev ekliyorsanız, bunun için birim testleri yazdığınızdan emin olun. Anlamlı birim testleri yazmak her zaman mümkün değildir - örneğin, grafikler bu şekilde test edilemez. Bu durumda, kodunuzu manuel olarak kapsamlı bir şekilde test ettiğinizden emin olun.
- **Kodunuzu açıklayın** — kodu neden yazdığınızı ve ne işe yaradığını açıklamak önemlidir. Gibi açıklamalarla çekme istekleri oluşturmayın: "Hata düzeltildi", "bir şeyler eklendi" vb. Kimseye yardımcı olmaz, bunun yerine uygun bir açıklama yazın.
- **Lisans** — `LICENSE.md dosyasının içeriğini herhangi bir yeni kaynak kod dosyasının en üstüne ekleyin. Her satır //` ile başlamalıdır (iki eğik çizgi ve ardından bir boşluk). Kendi telif hakkı satırınızı tarihlerle birlikte ekleyebilirsiniz, ancak lisansı değiştirmeden (MIT) tutmalısınız.

Editör için bir şey yazarken, `fyroxed` paketini kullanarak bağımsız sürümünü şu şekilde çalıştırabilirsiniz:

```shell
cargo run --package fyroxed
```

Bu şekilde editör herhangi bir eklenti olmadan çalışacak ve değişikliklerinizi hızlı bir şekilde test edebileceksiniz.
projesini oluşturun ve orada test edin.

## Belgelere katkıda bulunma

Dokümantasyon katkıları için ortak kurallar:

- **Her şeyi İngilizce yazın** — resmi API belgeleri ve ingilizce yazılmış [kitap](https://fyrox-book.github.io/). Kitap için bir çeviri oluşturmak istiyorsanız, kendi deponuzu oluşturmalısınız.
- **Kod örnekleri ekleyin** — kod parçacıkları diğer geliştiricilerin bir fonksiyonun/yöntemin nasıl kullanılacağını hızlı bir şekilde anlamalarına yardımcı olur.
- **Yazım denetleyicisini kullanın** — dokümanları temiz ve okunabilir tutun.
- **Uzmanlık** — belgelerini yazdığınız şeyi anladığınızdan emin olun. Sığ dokümanlar genellikle yanıltıcıdır ve bazen hiç dokümantasyon olmamasından daha da kötüdür.
