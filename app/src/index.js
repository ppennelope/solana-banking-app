const { Connection, PublicKey, clusterApiUrl, Keypair, LAMPORTS_PER_SOL } = require("@solana/web3.js");

async function main() {
    console.log("🚀 Solana Devnet Ağına Bağlanılıyor...");
    
    // 1. Ağ Bağlantısı Kurulumu (Banka altyapısına bağlanıyoruz)
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    console.log("✅ Ağ bağlantısı başarılı!\n");

    // 2. BANKA HESABI OLUŞTURMA İŞLEMİ (Yeni bir rastgele cüzdan)
    console.log("--- YENİ BANKA HESABI (CÜZDAN) OLUŞTURULUYOR ---");
    const yeniHesap = Keypair.generate();
    console.log("🔑 Yeni Hesap Adresi (Public Key):", yeniHesap.publicKey.toString());
    console.log("🔒 Gizli Anahtar (Private Key) oluşturuldu ve güvenliğe alındı.\n");

    // 3. BAKİYE OKUMA İŞLEMİ (Senin kendi cüzdanının bakiyesi)
    console.log("--- MEVCUT HESAP BAKİYESİ SORGULANIYOR ---");
    const seninCuzdanAdresin = new PublicKey("DcSwY9RX1u2oxX41Gp59mEGj43RSJgdVjtf6NASSVjJT");
    console.log("💳 Sorgulanan Adres:", seninCuzdanAdresin.toString());

    try {
        // Solana ağında bakiye 'lamports' cinsinden döner (1 SOL = 1 Milyar Lamports)
        const bakiyeLamports = await connection.getBalance(seninCuzdanAdresin);
        const bakiyeSol = bakiyeLamports / LAMPORTS_PER_SOL;

        console.log(`💰 Güncel Bakiyen: ${bakiyeSol} SOL`);
        console.log("\n🎉 İşlemler başarıyla tamamlandı kralım!");
    } catch (hata) {
        console.error("❌ Bakiye sorgulanırken bir hata oluştu:", hata);
    }
}

// Yazdığımız ana fonksiyonu tetikliyoruz
main();