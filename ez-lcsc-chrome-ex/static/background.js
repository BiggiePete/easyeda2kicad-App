chrome.runtime.onInstalled.addListener(() => {
  console.log("Extension Installed");

  // Set up any initial tasks when the extension is installed
});

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.action === "fetchData") {
    chrome.action.openPopup();
    // Make the fetch request from the service worker
    fetch('http://localhost:3030/api/getLCSC', {
      method: 'POST',
      body:JSON.stringify({C:message.code}),
      headers: {
        'Content-Type': 'application/json'
      },
      mode: 'cors', // Ensure the request respects CORS
    })
    .then(response => response.text())  // Handle response as text (or JSON if needed)
    .then(data => {
      console.log('Response:', data);
      sendResponse({ success: true, data: data });
    })
    .catch(error => {
      console.error('Error:', error);
      sendResponse({ success: false, error: error });
    });

    // Keep the message channel open for asynchronous response
    return true;
  }
});