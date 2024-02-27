<script lang="ts">
  import axios from 'axios';
  import { onMount } from 'svelte';

  interface Message {
    role: 'system' | 'user' | 'assistant';
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

  let messages: Message[] = [
    { role: "system", content: instructions },
    { role: "assistant", content: welcomeMessage }
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
    try {
      // Send the request to the proxy endpoint instead of the external API
      const response = await axios.post('/api', payload);

      
      // Assuming the server response structure matches what the client expects
      console.log(response.data);
      response.data.choices.map((choice: Choice) => {
        choice.messages.map((message: Message) => {
            messages = [...messages, { role: "assistant", content: message.content }];
          
        })
      }) 
      
    } catch (error) {
      console.error("Failed to send message through proxy:", error);
    }
  }

  onMount(() => {
    // Initialization logic if needed
  });
</script>


<style>
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
      <input id="instructions" type="text" bind:value={instructions}>
    </div>
    <div class="input-group">
      <label for="indexer">Search Index Name:</label>
      <input id="indexer" type="text" bind:value={indexer}>
    </div>
    <div class="upload-section">
      <label for="fileUpload">Upload files</label>
      <input id="fileUpload" type="file" multiple>
    </div>
  </div>

  <div class="box">
    <div class="chat-container">
      {#each messages as message}
        {#if message.role === "user" || message.role === "assistant"}
          <div class="message {message.role}">
            {message.content}
          </div>
         {/if}
       {/each}
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
      <!-- Add more buttons here as needed to match your reference design -->
    </div>
  </div>
</div>
