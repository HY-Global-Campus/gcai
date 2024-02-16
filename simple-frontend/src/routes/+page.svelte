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

  let messages: Message[] = [
    { role: "assistant", content: "Hello! How can I help today?" }
  ];
  let userInput: string = "";

  async function sendMessage(): Promise<void> {
   messages = [...messages, { role: "user", content: userInput }];
    const payload = {
      messages: messages 
    };

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
      
      userInput = ""; // Clear the input field
    } catch (error) {
      console.error("Failed to send message through proxy:", error);
    }
  }

  onMount(() => {
    // Initialization logic if needed
  });
</script>


<style>
  .chat-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
  }

  .message {
    padding: 10px;
    border-radius: 5px;
    margin: 5px;
  }

  .system {
    background-color: #f0f0f0;
  }

  .user {
    background-color: #e0e0e0;
  }

  .assistant {
    background-color: #d0d0d0;
  }
</style>

<div class="chat-container">
  {#each messages as message }
    <div class="message {message.role}">
      {message.content}
    </div>
  {/each}

  <input
    type="text"
    bind:value={userInput}
    on:keydown={(e) => e.key === 'Enter' && sendMessage()}
    placeholder="Type a message..."
  >
  <button on:click={sendMessage}>Send</button>
</div>
