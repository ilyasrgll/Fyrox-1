# Fyrox Motor Şablon Oluşturucu

Bu küçük yardımcı program, Fyrox Oyun Motoru için proje ve komut dosyası oluşturma işlemlerini gerçekleştirir.

## Kurulum

Install it via `cargo install`:

```shell
cargo install fyrox-template
```

## Yeni Proje Oluşturma

`fyrox-template init [--name <name> --style <style>]`

- `name` - yeni projenin adı (varsayılan `my_game`)
- `style` - varsayılan sahne türünü tanımlar, ya `2d` ya da `3d` (varsayılan `3d`)

Üç proje içeren bir çalışma alanı oluşturur:

- Oyun - oyun projeniz (kütüphane)
- Editör - oyununuzun eklenti olarak eklendiği editör
- Yürütücü - oyununuz için "koşucu".

Ayrıca her projeyi şablon kodla doldurur. Projenin ana amacı zaman miktarını azaltmaktır
yeni bir proje kurmak için gerekli olan.

<proje_adı>` ile yeni bir klasör oluşturacak ve sadece ikisi çalıştırılabilir üç proje içerecektir:

- `cargo run --package editor --release` - oyununuzu editör içinde çalıştırmak için.
- `cargo run --package executor --release` - oyununuzu bağımsız bir proje olarak çalıştırmak için. Ayrıca final üretecektir
  Oyununuzun bir mağazaya gönderilebilecek ikili kopyası.

### İpuçları

Oluşturulan projede özel bir şey yoktur, bu nedenle bunları istediğiniz gibi değiştirebilirsiniz.

## Yeni Komut Dosyaları Ekleme

`fyrox-template script [--name <name>]`

- `name` - betiğinizin adı (varsayılan `MyScript`)

Araç ayrıca sizin için komut dosyası iskeleti oluşturabilir ve gerekli tüm şablonlarla doldurabilir. Oluşturulan komut dosyaları
game/src` klasörüne eklenecektir, bu nedenle aracı oyununuzun kök klasöründen çalıştırmalısınız (kök Cargo.toml
bulunur).

Komut dosyasını modül ağacınıza gerekli konuma eklemeyi unutmayın, muhtemelen bazı küçük değişikliklere ihtiyacınız olacaktır
oluşturulan içerik için modern IDE'ler tarafından kolayca otomatikleştirilebilir.
