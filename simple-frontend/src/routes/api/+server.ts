import type { RequestHandler } from '@sveltejs/kit';
import axios from 'axios';
import dotenv from 'dotenv';

if (process.env.NODE_ENV !== 'production') {
	dotenv.config();
}

export const POST: RequestHandler = async ({ request }) => {
	const body = await request.json();

	// Define the URL of the external chat service
	const externalApiUrl = `${process.env.BACKEND_URL}/api/v2/azure_openai/chat_completion`;

	try {
		const response = await axios.post(externalApiUrl, body, {
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${process.env.BACKEND_TOKEN}` // Adjust as needed
			}
		});

		// Ensure the response matches the SvelteKit `Response` structure
		return new Response(JSON.stringify(response.data), {
			status: 200,
			headers: {
				'Content-Type': 'application/json'
			}
		});
	} catch (error) {
		if (axios.isAxiosError(error)) {
			// Properly handle Axios errors
			console.error('Failed to communicate with the chat service', error);
			const status = error.response?.status || 500;
			const errorMessage = error.response?.data || 'Failed to communicate with the chat service';
			return new Response(JSON.stringify({ error: errorMessage }), {
				status: status,
				headers: {
					'Content-Type': 'application/json'
				}
			});
		}

		// Handle non-Axios errors
		return new Response(JSON.stringify({ error: 'An unknown error occurred' }), {
			status: 500,
			headers: {
				'Content-Type': 'application/json'
			}
		});
	}
};
