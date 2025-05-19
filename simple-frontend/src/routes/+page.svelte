<script lang="ts">
	import axios from 'axios';
	import { onMount } from 'svelte';
	import { Router } from 'svelte-routing';
	import { writable } from 'svelte/store';
	import showdown from 'showdown';

	const converter = new showdown.Converter();

	interface Message {
		role: string;
		content: string;
		context?: MessageContext;
	}

	interface MessageContext {
		citations: MessageContextCitation[];
		intent: string;
	}

	interface MessageContextCitation {
		chunk_id: string;
		content: string;
		filepath: string;
		title: string;
		url: string;
	}

	interface ApiRequestBody {
		messages: Message[];
		deployment?: string;
		temperature?: number;
		top_p?: number;
		frequency_penalty?: number;
		presence_penalty?: number;
		max_tokens?: number;
		max_completion_tokens?: number;
		stop?: string;
		stream?: boolean;
		logit_bias?: object;
		user?: string;
		data_sources?: DataSource[];
		logprobs?: boolean;
		top_logprobs?: number;
		n?: number;
		parallel_tool_calls?: boolean;
		seed?: number;
		tools?: string[];
		tool_choice?: string;
	}

	interface ApiResponseBody {
		id: string;
		prompt_filter_results?: object;
		created: number;
		choices: Choice[];
		model: string;
		system_fingerprint?: string;
		object: string;
		usage?: object;
	}

	interface Choice {
		finish_reason: string;
		index: number;
		message: Message;
	}

	interface DataSource {
		type: string;
		parameters: DataSourceParameters;
	}

	interface DataSourceParameters {
		authentication: DataSourceAuthenticationParameters;
		index_name: string;
	}

	interface DataSourceAuthenticationParameters {
		type: string;
	}

	const showCitations = writable<MessageContext | null>(null);
	const expandedCitations = writable<Set<string>>(new Set());

	function openCitations(context: MessageContext | undefined) {
		if (context && context.citations) {
			showCitations.set(context);
		}
	}

	function closeCitations() {
		showCitations.set(null);
	}

	function toggleCitation(chunk_id: string) {
		expandedCitations.update((current) => {
			const newSet = new Set(current);
			if (newSet.has(chunk_id)) {
				newSet.delete(chunk_id);
			} else {
				newSet.add(chunk_id);
			}
			return newSet;
		});
	}

	let indexer: string = '';
	let userInput: string = '';
	let courseName: string = '';
	let assistantName: string = '';
	let welcomeMessage: string = 'Hello! How can I help today?';
	let instructions: string = 'You are an assistant that helps users find information.';
	let showSearchIndex: boolean = false;
	let loading: boolean = false;

	let messages: Message[] = [{ role: 'system', content: instructions }];

	async function sendMessage(): Promise<void> {
		if (!userInput.trim()) return;

		messages[0].content = instructions;
		messages = [...messages, { role: 'user', content: userInput }];
		let payload: ApiRequestBody = {
			messages: messages,
			data_sources:
				indexer == ''
					? undefined
					: [
							{
								type: 'azure_search',
								parameters: {
									authentication: {
										type: 'api_key'
									},
									index_name: indexer
								}
							}
						]
		};

		userInput = ''; // Clear the input field
		loading = true;

		try {
			const response = await axios.post(`/api`, payload);
			const responseBody: ApiResponseBody = response.data;
			console.log('Response from server:', responseBody);

			if (responseBody.choices && responseBody.choices.length > 0) {
				responseBody.choices.forEach((choice: Choice) => {
					if (choice.message) {
						choice.message.content = converter.makeHtml(choice.message.content);
						messages = [...messages, choice.message];
					} else {
						messages = [
							...messages,
							{ role: 'assistant', content: '[No messages returned from assistant]' }
						];
					}
				});
			} else {
				messages = [
					...messages,
					{ role: 'assistant', content: '[No choices returned in response]' }
				];
			}
		} catch (error) {
			console.error('Failed to send message through proxy:', error);
			messages = [...messages, { role: 'error', content: '[Error: Unable to connect to server]' }];
		}
		loading = false;
	}

	let dots = '';

	onMount(() => {
		const interval = setInterval(() => {
			if (dots.length < 4) {
				dots += '.';
			} else {
				dots = '.';
			}
		}, 500);

		return () => {
			clearInterval(interval);
		};
	});
</script>

<Router>
	<div class="banner">
		<h1>AI Creator</h1>
		<div class="logo"></div>
	</div>
	<div class="container">
		<div class="box">
			<div class="input-group">
				<label for="courseName">Course name</label>
				<input id="courseName" type="text" bind:value={courseName} />
			</div>
			<div class="input-group">
				<label for="assistantName">Assistant name</label>
				<input id="assistantName" type="text" bind:value={assistantName} />
			</div>
			<div class="input-group">
				<label for="welcomeMessage">Welcome message</label>
				<input id="welcomeMessage" type="text" bind:value={welcomeMessage} />
			</div>
			<div class="input-group">
				<label for="instructions">Instructions</label>
				<textarea rows="5" bind:value={instructions} placeholder="Enter instructions here..."
				></textarea>
			</div>
			<div class="input-group">
				<div>
					<label for="showSearchIndex">Use own data</label>
					<input id="showSearchIndex" type="checkbox" bind:checked={showSearchIndex} />
				</div>
				{#if showSearchIndex}
					<label for="indexer">Search Index Name:</label>
					<input id="indexer" type="text" bind:value={indexer} />
				{/if}
			</div>
			<div class="upload-section">
				<label for="fileUpload">Upload files</label>
				<input id="fileUpload" type="file" multiple />
			</div>
		</div>

		<div class="box">
			<h2>Test your assistant</h2>
			<div class="chat-container">
				{#each messages as message}
					{#if message.role === 'assistant'}
						<div class="message assistant">
							{@html message.content}
							{#if message.context && message.context.citations.length > 0}
								<button on:click={() => openCitations(message.context)}>View Citations</button>
							{/if}
						</div>
					{:else if message.role === 'user'}
						<div class="message user">
							{message.content}
						</div>
					{:else}
						<div class="message error">
							{message.content}
						</div>
					{/if}
				{/each}
			</div>

			{#if $showCitations}
				<div class="popup">
					<h2>Citations:</h2>
					<ul>
						{#each $showCitations.citations as citation}
							<li>
								<button class="toggle-button" on:click={() => toggleCitation(citation.chunk_id)}>
									{$expandedCitations.has(citation.chunk_id) ? '−' : '+'}
								</button>
								<strong>File:</strong>
								{citation.filepath} — <br /><strong>Chunk ID:</strong>
								{citation.chunk_id}
								{#if $expandedCitations.has(citation.chunk_id)}
									<div class="citation-details">
										<p><strong>Content:</strong> {citation.content}</p>
										<p><strong>Title:</strong> {citation.title}</p>
										<p><strong>Filepath:</strong> {citation.filepath}</p>
										<p><strong>URL</strong> {citation.url}</p>
									</div>
								{/if}
							</li>
						{/each}
					</ul>
					<button on:click={closeCitations}>Close</button>
				</div>
			{/if}
			<div class="input-group">
				<input
					type="text"
					bind:value={userInput}
					on:keydown={(e) => e.key === 'Enter' && sendMessage()}
					placeholder="Type a message..."
				/>
			</div>
			<div class="button-group">
				<button on:click={sendMessage} disabled={loading}>Send</button>
			</div>
		</div>
	</div>
</Router>

<style>
	@font-face {
		font-family: 'Gotham Narrow';
		src: url('fonts/Gotham-Narrow-Font-Family/GothamNarrow-Regular.woff') format('woff');
		font-weight: normal;
		font-style: normal;
	}

	@font-face {
		font-family: 'Gotham Narrow';
		src: url('fonts/Gotham-Narrow-Font-Family/GothamNarrow-Bold.woff') format('woff');
		font-weight: bold;
		font-style: normal;
	}

	@font-face {
		font-family: 'Gotham Narrow';
		src: url('fonts/Gotham-Narrow-Font-Family/GothamNarrow-Italic.woff') format('woff');
		font-weight: normal;
		font-style: italic;
	}

	@font-face {
		font-family: 'Gotham Narrow';
		src: url('fonts/Gotham-Narrow-Font-Family/GothamNarrow-BoldItalic.woff') format('woff');
		font-weight: bold;
		font-style: italic;
	}

	:global(body) {
		margin: 0;
		padding: 0;
		font-family: Gotham Narrow;
	}

	.banner {
		background-color: #333;
		color: white;
		height: 100px;
		width: 100%;
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		padding: 10px 0px;
	}
	h1 {
		padding: 0 20px;
	}
	.logo {
		height: 100%;
		width: 300px;
		background-image: url('HY_logo_W.png');
		background-size: contain;
		background-repeat: no-repeat;
	}
	.container {
		display: grid;
		grid-template-columns: 25% 75%;
		gap: 20px;
		padding: 20px;
		height: calc(100vh - 200px);
	}

	.box {
		border: 1px solid #ccc;
		padding: 20px;
		margin: 10px;
		width: 95%;
	}

	.input-group {
		display: flex;
		flex-direction: column;
		margin-bottom: 10px;
	}

	.input-group label {
		font-weight: bold;
		margin-bottom: 5px;
	}

	.input-group input {
		padding: 8px;
		border: 1px solid #ccc;
		border-radius: 5px;
	}
	.input-group textarea {
		height: auto;
		min-height: 80px;
		padding: 10px;
		border: 1px solid #ccc;
		border-radius: 4px;
		resize: none;
		overflow-y: auto;
	}

	.input-group input[type='checkbox'] {
		margin-right: 5px;
		align-self: flex-start;
	}

	.chat-container {
		border: 1px solid #ccc;
		padding: 20px;
		border-radius: 10px;
		height: 60vh;
		max-height: 70%;
		overflow-y: scroll;
		display: flex;
		flex-direction: column;
	}

	.button-group {
		display: flex;
		gap: 10px;
		margin-top: 10px;
	}

	button {
		padding: 10px 15px;
		border: none;
		border-radius: 5px;
		cursor: pointer;
		background-color: #333;
		color: white;
	}

	button:disabled {
		background-color: #888;
		cursor: not-allowed;
	}

	.message {
		padding: 10px 20px;
		border-radius: 18px;
		margin-bottom: 10px;
		max-width: 60%;
		word-wrap: break-word;
	}

	.message.user {
		background-color: #e0e0e0;
		align-self: flex-end;
		margin-left: auto;
	}

	.message.assistant {
		background-color: #d0d0d0;
		align-self: flex-start;
		margin-right: auto;
	}

	.message.error {
		background-color: #f8d7da;
		border: 1px solid #f5c2c7;
		color: #721c24;
		align-self: flex-start;
		margin-right: auto;
	}

	.message.loading {
		color: #555;
		font-style: italic;
	}
	.message button {
		padding: 5px;
		font-size: 12px;
		cursor: pointer;
	}

	.popup {
		position: fixed;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background-color: white;
		border: 1px solid #ccc;
		padding: 20px;
		border-radius: 8px;
		z-index: 1000;
		width: 70%;
		max-height: 70%;
		overflow: scroll;
	}

	.popup button {
		margin-top: 10px;
	}

	.popup ul {
		list-style: none;
		padding: 0;
	}

	.upload-section {
		display: flex;
		align-items: center;
		gap: 10px;
	}
</style>
