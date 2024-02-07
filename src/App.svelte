<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import bs58 from 'bs58';

	let providedCode = '';
	let passwordString = '';
	let privateKey = '';
	let bitcoinPrivateKey = '';
	let error = '';
	let warning = '';
	let isPasswordVisible = true;
	let isButtonDisabled = true;



	function providedCodeInput() {
		if (providedCode.length < 16) {
			warning += ' Provided codeは16文字以上である必要があります';
		}
	}

	function passwordStringInput() {
		if (passwordString.length < 8) {
			warning += ' Passwordは8文字以上である必要があります';
		}
	}

	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(text).then(
			() => {
				console.log('クリップボードにコピーしました');
			},
			(err) => {
				console.error('クリップボードへのコピーに失敗しました:', err);
			}
		);
	}

	async function handleFormSubmit() {
		validateInput(); // Base58形式の検証
		providedCodeInput(); // Provided codeの長さ検証
		passwordStringInput(); // Passwordの長さ検証

		// 他の条件が満たされている場合のみ、バックエンドの関数を呼び出す
		if (warning === '') {
			try {
				const result = await invoke('generate_private_key', { providedCode, passwordString });
				if (typeof result === 'string') {
					privateKey = result; // コンポーネントのステートを更新
				} else {
					console.error('Result is not a string');
				}
			} catch (err) {
				error = (err as Error).message;
			}
		}
	}

	async function handleGenerateBitcoinKey() {
		if (!isButtonDisabled) {
			try {
				const result = await invoke('generate_bitcoin_private_key', {
					providedCode,
					passwordString
				});
				if (typeof result === 'string') {
					bitcoinPrivateKey = result; // コンポーネントのステートを更新
				} else {
					console.error('Result is not a string');
				}
			} catch (err) {
				error = (err as Error).message;
			}
		}
	}

	// 入力検証を行う関数
	function validateInput() {
		// Base58形式の検証
		let isBase58Valid = true;
		try {
			bs58.decode(providedCode);
			bs58.decode(passwordString);
			warning = '';
		} catch (e) {
			warning = 'Base58の文字を入力してください';
			isBase58Valid = false;
		}

		// Provided codeとPasswordの長さ検証
		if (providedCode.length < 16) {
			warning += ' Provided codeは16文字以上である必要があります';
			isButtonDisabled = true;
		} else if (passwordString.length < 8) {
			warning += ' Passwordは8文字以上である必要があります';
			isButtonDisabled = true;
		} else if (isBase58Valid) {
			isButtonDisabled = false; // すべての条件を満たす場合、ボタンを活性化
		}
	}
</script>

<main class="background">
	<div class="container">
		<h1>
			Block Hand key generator
		</h1>
		<div class="password-input">
			{#if isPasswordVisible}
				<input
					class="input-field"
					bind:value={providedCode}
					type="text"
					placeholder="Input cord inside ring"
					on:input={validateInput}
				/>
			{:else}
				<input
					class="input-field"
					bind:value={providedCode}
					type="password"
					placeholder="Input cord inside ring"
					on:input={validateInput}
				/>
			{/if}
		</div>

		<div class="password-input">
			{#if isPasswordVisible}
				<input
				    class="input-field"
					bind:value={passwordString}
					type="text"
					placeholder="Input password of at least 8 characters"
					on:input={validateInput}
				/>
			{:else}
				<input
				    class="input-field"
					bind:value={passwordString}
					type="password"
					placeholder="Input password of at least 8 characters"
					on:input={validateInput}
				/>
			{/if}

		</div>

		<button on:click={handleFormSubmit} disabled={isButtonDisabled}>Generate Keys</button>

		{#if warning}
			<p class="warning">{warning}</p>
		{/if}

		{#if privateKey}
			<p class="key">Private Key: {privateKey}</p>
			<button on:click={() => copyToClipboard(privateKey)}>Copy to Clipboard</button>
		{/if}
		{#if error}
			<p class="error">{error}</p>
		{/if}

		<button on:click={handleGenerateBitcoinKey} disabled={isButtonDisabled}
			>Generate Bitcoin Address</button
		>

		{#if bitcoinPrivateKey}
			<p class="key">Bitcoin Private Key: {bitcoinPrivateKey}</p>
			<button on:click={() => copyToClipboard(bitcoinPrivateKey)}>Copy to Clipboard</button>
		{/if}
	</div>
</main>

<style>
	:root {
		--primary-color: #aca99b;
		--secondary-color: #d8d4ce;
		--background-color: #e8e8e9;
		--base-color: #c0bcaf;
		--accent-color: #8e8c81;
	}

	.container {
		display: flex; /* Flexboxを有効にします */
		flex-direction: column; /* 子要素を縦方向に配置します */
		justify-content: center; /* 子要素を水平方向で中央に配置します */
		align-items: center; /* 子要素を垂直方向で中央に配置します */
		margin: 0 auto; /* ウィンドウ全体を中央に配置します */
		font-family: 'Courier New', monospace;
		background-image: linear-gradient(to right, var(--primary-color), #ffffff);
		width: 100%;
		/* 高さを常に画面いっぱいにする */
		height: 100vh;

	}

	.background {

		background-image: linear-gradient(
			to right,

		); /* グラデーション適用 */
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		border-radius: 10px;
		display: flex; /* Flexboxを有効にします */
		flex-direction: column; /* 子要素を縦方向に配置します */
		justify-content: center; /* 子要素を水平方向で中央に配置します */
		align-items: center; /* 子要素を垂直方向で中央に配置します */
	}
     h1 {
		font-size: 1.5rem;
		margin-bottom: 3rem;
	 }
	input {
		width: 80%;
		padding: 10px;
		margin-bottom: 20px;
		background-color: transparent;
		border: none;
		border-bottom: 2px solid #8e8c81;
		font-size: 1rem;
		color: #8e8c81;
	}

	input::placeholder {
		color: #aca99b;
	}

	input:focus {
    border-color: #aca99b; /* フォーカス時のボーダー色を変更 */
    box-shadow: 0 0 3px #aca99b; /* フォーカス時のシャドウを追加 */
}


	.warning,
	.error {
		color: accent-color;
		margin-top: 1rem;
	}

	button {
    background-color: var(--primary-color);
    color: #ffffff;
    padding: 10px 15px;
    border: none;
    border-radius: 5px;
    transition: background-color 0.3s ease;
	 margin-bottom: 20px;
}


	button:disabled {
    background-color: #838383;
    cursor: not-allowed;
}

	button:hover {
    background-color: darken(var(--primary-color), 10%); /* ホバー時の背景色を暗く */
}

   .password-input{
   width: 90%;
   display: flex;
   justify-content: center;
   align-items: center;
   margin-bottom: 1.5rem;

   }
   .input-field{
	background-color:#e0ddd8}

	.key{
		color: var(--accent-color);
	}
</style>
