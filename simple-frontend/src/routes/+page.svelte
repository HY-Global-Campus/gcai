<script lang="ts">
  import axios from 'axios';
  import { onMount } from 'svelte';
  import { base } from '$app/paths';

  interface Message {
    role: 'system' | 'user' | 'assistant' | 'tool';
    content: string;
  }

  interface Choice {
    messages: Message[];
    id: string;
    created: number;
  }

  let indexer: string = "";
  let userInput: string = "";
  let courseName: string = "";
  let assistantName: string = "";
  let welcomeMessage: string = "Hello! How can I help today?";
  let instructions: string = "You are an assistant that helps users find information.";
  let showSearchIndex: boolean = false;
  let loading: boolean = false;

  let messages: Message[] = [
    { role: "system", content: instructions },
  ];

  async function sendMessage(): Promise<void> {
    messages[0].content = instructions;
    messages = [...messages, { role: "user", content: userInput }];
    let payload: object;
    if (indexer != "") {
      payload = {
        messages: messages, 
        azure_search_index_name: indexer
      };
    } else {
      payload = {
        messages: messages 
      };
    }
    console.log(payload);

    userInput = ""; // Clear the input field
    loading = true

    try {
      // Send the request to the proxy endpoint instead of the external API
      const response = await axios.post(`${base}/api`, payload);

      
      // Assuming the server response structure matches what the client expects
      console.log(response.data);
      response.data.choices.map((choice: Choice) => {
        choice.messages.map((message: Message) => {
            messages = [...messages, { role: message.role, content: message.content }];
          
        })
      }) 
      
    } catch (error) {
      console.error("Failed to send message through proxy:", error);
    }
    loading = false;
  }

  let dots = ''; // Initial state with no dots

  onMount(() => {
    const interval = setInterval(() => {
            // Update dots state, cycling through ., .., ...
            if (dots.length < 4) {
                dots += '.';
            } else {
              dots = '.';
            }
        }, 500); // Change dot state every 500ms

        return () => {
            clearInterval(interval); // Clean up interval on component destroy
        };
    });
</script>


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
    width: 100vw;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    padding: 10px 20px;
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
    grid-template-columns: 1fr 1fr;
    gap: 20px;
    padding: 20px;
  }

  .box {
    border: 1px solid #ccc;
    border-radius: 10px;
    padding: 20px;
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

  .input-group input[type="checkbox"] {
    margin-right: 5px;
    align-self: flex-start;
  }

  .chat-container {
    border: 1px solid #ccc;
    padding: 20px;
    border-radius: 10px;
    height: 300px;
    overflow-y: auto;
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

  .upload-section {
    display: flex;
    align-items: center;
    gap: 10px;
  }


</style>
<div class="banner">
  <h1>AI Creator</h1>
  <div class="logo"></div>
</div>
<div class="container">
  <div class="box">
    <div class="input-group">
      <label for="courseName">Course name</label>
      <input id="courseName" type="text" bind:value={courseName}>
    </div>
    <div class="input-group">
      <label for="assistantName">Assistant name</label>
      <input id="assistantName" type="text" bind:value={assistantName}>
    </div>
    <div class="input-group">
      <label for="welcomeMessage">Welcome message</label>
      <input id="welcomeMessage" type="text" bind:value={welcomeMessage}>
    </div>
    <div class="input-group">
      <label for="instructions">Instructions</label>
      <textarea rows="5" bind:value={instructions} placeholder="Enter instructions here..."></textarea>
    </div>
    <div class="input-group">
      <div>
        <label for="showSearchIndex">Use own data</label>
        <input id="showSearchIndex" type="checkbox" bind:checked={showSearchIndex}>
      </div>
      {#if showSearchIndex}
        <label for="indexer">Search Index Name:</label>
        <input id="indexer" type="text" bind:value={indexer}>
      {/if}
    </div>
    <div class="upload-section">
      <label for="fileUpload">Upload files</label>
      <input id="fileUpload" type="file" multiple>
    </div>
  </div>

  <div class="box">
    <h2>Test your assistant</h2>
    <div class="chat-container">
      <div class="message assistant">
        {welcomeMessage}
      </div>
      {#each messages as message}
        {#if message.role === 'user' || message.role === 'assistant'}
          <div class="message {message.role}">
            {message.content}
          </div>
         {/if}
       {/each}
       {#if loading}
        <div class="message assistant">
          {dots}
        </div>
      {/if}
    </div>
    <div class="input-group">
      <input
        type="text"
        bind:value={userInput}
        on:keydown={(e) => e.key === 'Enter' && sendMessage()}
        placeholder="Type a message..."
      >
    </div>
    <div class="button-group">
      <button on:click={sendMessage}>Send</button>
    </div>
  </div>
</div>
