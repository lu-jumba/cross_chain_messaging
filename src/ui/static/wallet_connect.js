import WalletConnect from "@walletconnect/client";
import QRCodeModal from "@walletconnect/qrcode-modal";

// WalletConnect logic
async function connectWalletConnect() {
    const connector = new WalletConnect({
        bridge: "https://bridge.walletconnect.org",
        qrcodeModal: QRCodeModal,
    });

    if (!connector.connected) {
        // Create a new session
        await connector.createSession();
    }

    return new Promise((resolve) => {
        connector.on("connect", (error, payload) => {
            if (error) {
                throw error;
            }

            const { accounts } = payload.params[0];
            resolve(accounts[0]); // return the first account
        });
    });
}

// Export this function so that Rust can call it
export { connectWalletConnect };
