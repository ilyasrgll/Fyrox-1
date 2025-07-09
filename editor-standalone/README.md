# FyroxEd (bağımsız)

**UYARI:** Editörün bağımsız sürümü desteklenmiyor,editörün tam gücünü kullanmak için [proje şablonu oluşturucuyu](https://fyrox-book.github.io/fyrox/beginning/scripting.html) kullanın. Bağımsız sürüm eklentileri ve komut dosyalarını desteklemez, sonraki sürümlerde güncellenmeyecektir!

FyroxEd'in bağımsız bir sürümü - [Fyrox motorunun](https://github.com/FyroxEngine/Fyrox) yerel editörü. Bağımsız sürüm yalnızca sahne oluşturmanıza ve düzenlemenize izin verir, ancak **oyununuzu düzenleyicide çalıştırmaz**. Editörün farklı şekillerde nasıl kullanılacağını öğrenmek için lütfen [kitaba](https://fyrox-book.github.io/) bakınız.

## Nasıl kurulur ve çalıştırılır

crates.io'dan en son kararlı **standalone** sürümünü yüklemek için kullanın:

```shell
cargo install fyroxed
```

Bundan sonra, basitçe çağırarak editörü çalıştırabilirsiniz:

```shell
fyroxed
```

Eğer Linux kullanıyorsanız, lütfen aşağıdaki bağımlılıkların kurulu olduğundan emin olun:

```shell
sudo apt install libxcb-shape0-dev libxcb-xfixes0-dev libxcb1-dev libxkbcommon-dev libasound2-dev
```

## Controls

- [Click] - Seçmek
- [W][S][A][D] - Kamerayı hareket ettir
- [Space][Q]/[E] - Kamerayı Kaldır/Alçalt
- [1] - Etkileşim modunu seçin
- [2] - Etkileşim modunu taşıma
- [3] - Ölçek etkileşim modu
- [4] - Etkileşim modunu döndür
- [Ctrl]+[Z] - Geri al
- [Ctrl]+[Y] - Yeniden Yap]()
