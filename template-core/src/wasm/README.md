## Oluşturma talimatları

1. rustup'ta `wasm32-unknown-unknown` hedefinin yüklü olduğundan emin olun (değilse, şunu yapın: `rustup target add wasm32-unknown-unknown`)
2. Wasm-pack`in kurulu olduğundan emin olun (kurulu değilse, şunu yapın: `cargo install wasm-pack`)
3. Yürütücüyü oluşturmak için şunu yapın: `wasm-pack build --target web --release`

## Localhost'ta oyun nasıl çalıştırılır

1. basic-http-server`ın kurulu olduğundan emin olun (kurulu değilse: `cargo install basic-http-server`).
2. Varlıkları `executor-wasm` dizinine klonlayın. Alternatif olarak, `Cargo.toml` ve `src` dizini dışındaki her şeyi klonlayın
   projenizin kök dizinine (`../`) yerleştirin.
3. Executor-wasm`dizinindeki (veya alternatif yol kullandıysanız kök klasördeki)`basic-http-server`ı çalıştırın.

Her şey başarılı olduysa, http://localhost:4000/ adresinde bir web tarayıcısı açın, "Başlat" düğmesine tıklayın ve oyununuz yüklenecektir.
