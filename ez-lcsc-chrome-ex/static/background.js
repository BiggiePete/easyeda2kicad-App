chrome.runtime.onInstalled.addListener(() => {
	console.log('Extension Installed');

	// Set up any initial tasks when the extension is installed
});

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
	switch (message.action) {
		case 'getHealth':
			fetch('http://localhost:3030/api/getHealth', {
				method: 'GET',
				body: JSON.stringify(),
				headers: {
					'Content-Type': 'application/json'
				},
				mode: 'cors' // Ensure the request respects CORS
			})
				.then((response) => response.json()) // Parse the JSON response
				.then((data) => {
					console.log('Response:', data);
					sendResponse(data);
				})
				.catch((error) => {
					console.error('Error:', error);
					sendResponse(null);
				});
			return true; // Important: keep message channel open
		case 'getProjectList':
			fetch('http://localhost:3030/api/getProjectList', {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json'
				},
				mode: 'cors' // Ensure the request respects CORS
			})
				.then((response) => response.json()) // Parse the JSON response
				.then((data) => {
					console.log('Response:', data);
					sendResponse(data);
				})
				.catch((error) => {
					console.error('Error:', error);
					sendResponse(null);
				});
			return true;
		case 'add2Project':
			return true; // Important: keep message channel open
		case 'createNewProject':
			fetch('http://localhost:3030/api/createNewProject', {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json'
				},
				mode: 'cors' // Ensure the request respects CORS
			})
				.then((response) => response.json()) // Parse the JSON response
				.then((data) => {
					console.log('Response:', data);
					sendResponse(data);
				})
				.catch((error) => {
					console.error('Error:', error);
					sendResponse(null);
				});
			return true; // Important: keep message channel open
		case 'openPOPUP':
			chrome.action.openPopup();
			break;
		default:
			break;
	}
});
