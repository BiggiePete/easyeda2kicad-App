chrome.runtime.onInstalled.addListener(() => {
	console.log('Extension Installed');

	// Set up any initial tasks when the extension is installed
});

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
	console.log('PING!');
	if (message.action === 'openPOPUP') {
		chrome.action.openPopup();
		// Make the fetch request from the service worker
		// Keep the message channel open for asynchronous response
		return true;
	} else if (message.action === 'sendLCSCCODE') {
		fetch('http://localhost:3030/api/getLCSC', {
			method: 'POST',
			body: JSON.stringify({ C: message.code }),
			headers: {
				'Content-Type': 'application/json'
			},
			mode: 'cors' // Ensure the request respects CORS
		})
			.then((response) => response.text()) // Handle response as text (or JSON if needed)
			.then((data) => {
				console.log('Response:', data);
			})
			.catch((error) => {
				console.error('Error:', error);
			});
	} else if (message.action === 'getProjectList') {
		console.log('message Sending');
		fetch('http://localhost:3030/api/getProjectList', {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			},
			mode: 'cors' // Ensure the request respects CORS
		})
			.then((response) => response.text()) // Handle response as text (or JSON if needed)
			.then((data) => {
				console.log('Response:', data);
			})
			.catch((error) => {
				console.error('Error:', error);
			});
	}
});
