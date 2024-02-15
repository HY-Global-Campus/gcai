<script lang="ts">
  import axios from 'axios';
  import { onMount } from 'svelte';

  interface Message {
    role: 'system' | 'user' | 'assistant';
    content: string;
  }

  let messages: Message[] = [
    { role: "assistant", content: "This is test message" }
  ];
  let userInput: string = "";

  async function sendMessage(): Promise<void> {
    const payload = {
      messages: [
        { role: "system", content: "You are an AI assistant that helps people find information." },
        { role: "user", content: userInput }
      ]
    };

    try {
      // Send the request to the proxy endpoint instead of the external API
      const response = await axios.post('/api', payload);
      messages.push({ role: "user", content: userInput });
      // Assuming the server response structure matches what the client expects
      console.log(response.data);
      const botResponse: Message | undefined = response.data.choices[0].message;
      if (botResponse) {
        messages.push({ role: "assistant", content: botResponse.content });
      }
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
