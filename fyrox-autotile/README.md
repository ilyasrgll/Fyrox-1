# fyrox-autotile

Düzenli döşeme ızgaraları ile kullanım için otomatik döşeme algoritmalarının bir uygulaması. Bu içerir:

- Karo harita düzenlemesi için etkileşimli arazi tabanlı otomatik döşeme. Kullanıcı her hücre için geniş bir karo kategorisi seçer. Bu kategorilere _arazi_ adı verilir ve her arazide çok sayıda karo bulunabilir. Algoritma, karoların doğal bir şekilde birbirine uyması için her hücre için arazisinden belirli karoları seçer, böylece kullanıcıyı eşleşen karoları manuel olarak seçmek zorunda bırakmaz.

  Algoritma, aynı amaca yönelik bir Godot kütüphanesine dayanmaktadır: https://github.com/dandeliondino/terrain-autotiler

- Wave Function Collapse kullanarak rastgele karo üretimi. Amacın bir sanatçı için sıkıcı işleri otomatikleştirmek olduğu arazi tabanlı otomatik döşemenin aksine, Wave Function Collapse'in amacı, rastgeleleştirme yoluyla yaratıcı döşeme düzenleri oluşturarak sanatçıyı tamamen değiştirmektir.

  Algoritma, aynı amaca yönelik bir C++ kütüphanesi olan fast-wfc'ye dayanmaktadır: https://github.com/math-fehr/fast-wfc

Bu algoritmalar, _herhangi bir_ düzenli ızgara ile çalışacak şekilde genel olarak tasarlanmıştır. Kare hücreler, altıgen hücreler ve 3D ızgaralarda da aynı şekilde çalışmalıdırlar. Kullanıcı, hücreleri tanımlamak için kullanılan koordinat sistemini ve hangi hücrelerin hangi diğer hücrelere komşu olduğunu belirler.
