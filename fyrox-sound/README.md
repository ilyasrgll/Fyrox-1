# fyrox-ses

Rust ile yazılmış oyunlar ve etkileşimli uygulamalar için ses kütüphanesi.

**NOT:** Bu sandığın adında `fyrox` öneki olmasına rağmen, herhangi bir sorun olmadan ayrı olarak kullanılabilir.

## Temel özellikler

- Genel ve Uzamsal ses kaynakları.
- Büyük sesler için dahili akış.
- Ham örnekleri oynatma desteği.
- WAV formatı desteği (sıkıştırılmamış).
- Vorbis/ogg desteği (kullanarak [lewton](https://crates.io/crates/lewton)).
- mükemmel konumlandırma ve binaural efektler için [HRTF](https://en.wikipedia.org/wiki/Head-related_transfer_function) desteği.
- Yankı efekti.

## Örnekler

Örnekler `./examples` içinde bulunabilir. Örnekleri `--release` bayrağı ile çalıştırdığınızdan emin olun, `debug` sürümü çok yavaştır ve çıkış sesinin yırtılmasına neden olabilir.

## Desteklenen İşletim Sistemi

- Windows (DirectSound)
- Linux (alsa)
- macOS (CoreAudio)
- WebAssembly (WebAudio)
- Android (AAudio, API Level 26+)

## HRTF

Bu kütüphane tam HRTF desteğine sahiptir, [IRCAM](http://recherche.ircam.fr/equipes/salles/listen/) HRIR veritabanı kullanılarak oluşturulan HRIR kürelerini kullanır. HRIR küreleri C++ dilinde yazılmış küçük bir araç kullanılarak üretilir - [hrir_sphere_builder](https://github.com/mrDIMAS/hrir_sphere_builder). Size uygun HRTF'yi bulmak çok önemlidir çünkü bunlar çok bireyseldir ve genel algı tamamen doğru HRTF kullanımı ile tanımlanır.

## Katkılar

Her türlü katkı çok takdir edilmektedir! Projeye nasıl yardımcı olabileceğinizi görmek için `Sorunlar` sayfasını kontrol edin.

## Lisans

MIT

## Referanslar

Bu insanların çalışmaları olmasaydı bu kütüphane asla oluşturulamazdı. Hepinize teşekkür ederim!

1. [Dijital sinyal işleme ve filtreler](https://ccrma.stanford.edu/~jos/filters/)
2. [Fiziksel Ses Sinyali İşleme](https://ccrma.stanford.edu/~jos/pasp/)
3. Hannes Gamper, "Azimutta kafa ile ilgili transfer fonksiyonu enterpolasyonu, yükseklik ve mesafe", TAmerika Akustik Derneği Dergisi 134, EL547 (2013); doi: 10.1121/1.4828983
4. Fábio P. Freeland, Luiz W. P. Biscainho, Paulo S. R. Diniz, "Baş ile ilgili transfer fonksiyonunun (HRTFS) enterpolasyonu: Çok kaynaklı bir yaklaşım"
5. [IRCAM Head ile ilgili dürtü yanıtı veritabanı](http://recherche.ircam.fr/equipes/salles/listen/)
6. [Reverb](https://ccrma.stanford.edu/~jos/pasp/Freeverb.html)
7. [Örtüşme-ekleme konvolüsyonu](https://en.wikipedia.org/wiki/Overlap%E2%80%93add_method) - Dürtü yanıtı değiştiğinde segment sınırında önemli bozulmalar nedeniyle artık kullanılmamaktadır.
8. [Örtüşme-kaydetme konvolüsyonu](https://dsp-nbsphinx.readthedocs.io/en/nbsphinx-experiment/nonrecursive_filters/segmented_convolution.html) - dürtü yanıtı değiştiğinde çok daha iyi çalışır, sadece az çok kabul edilebilir olan faz kayması sorunları vardır.
9. [OpenAL Spesifikasyonu](https://www.openal.org/documentation/openal-1.1-specification.pdf) - mesafe modelleri ve genel tasarım hususları.
10. http://csoundjournal.com/issue9/newHRTFOpcodes.html - hrtf renderer'da tıklamaları kaldırmak için bazı fikirler
11. https://phaidra.kug.ac.at/open/o:11024
