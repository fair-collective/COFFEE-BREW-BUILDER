#[tauri::command]
async fn generate_site(prompt: String) -> Result<String, String> {
    let code = format!(
        r#"
<!DOCTYPE html>
<html>
<head>
  <title>Coffee Brew — {0}</title>
  <meta name="description" content="Built with Coffee Brew Builder — Free AI Web3 Tool">
  <style>
    body {{ font-family: 'Arial'; background: #f9f9f9; }}
    .brew {{ background: #228B22; color: white; padding: 10px; border-radius: 8px; }}
  </style>
</head>
<body>
  <div class="brew">
    <h1>Coffee Brew — {0}</h1>
    <p>Built in 10 minutes. Accepts crypto. NFT loyalty included.</p>
    <button onclick="connectWallet()">Pay with Crypto</button>
  </div>
  <script src="https://cdn.ethers.io/lib/ethers-6.7.umd.min.js"></script>
  <script>
    async function connectWallet() {{
      if (window.ethereum) {{
        await window.ethereum.request({{ method: 'eth_requestAccounts' }});
        alert('Wallet connected! Brew on!');
      }}
    }}
  </script>
</body>
</html>
        "#,
        prompt
    );
    Ok(code)
}
