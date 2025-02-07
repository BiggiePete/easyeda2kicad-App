function findAddToCartButton() {
  return document.querySelector('div.mt-5[data-v-8acf6358] button.v-btn.primary[data-v-8acf6358]');
}

function getCCode(){
  function findAllCSharps() {
    // Find all the spans with class 'major2--text'
    const cSharpSpans = document.querySelectorAll('span.major2--text');
    
    // Map through the NodeList and extract text content from each span
    const cSharpValues = Array.from(cSharpSpans).map(span => span.textContent.trim());
  
    return cSharpValues;
  }
  return findAllCSharps().filter((v)=>{
    return v.charAt(0) === 'C' && !isNaN(v.slice(1)) && v.slice(1).length > 0
  })[0]
}

// Function to add our custom button next to the Add to Cart button
function addCustomButton() {
  const addToCartBtn = findAddToCartButton();
  if (!addToCartBtn) {
      console.log('Target button not found, retrying...');
      setTimeout(addCustomButton, 1000); // Retry after 1 second
  }
console.log(getCCode())
  // // Create our custom button with similar styling
  const customButton = document.createElement('button');
  customButton.textContent = 'Add To Project';
  customButton.className = 'v-btn v-btn--block v-btn--is-elevated v-btn--has-bg theme--light v-size--default secondary mt-5 add-2-project-tag';
  customButton.style.height = '42px';
  customButton.style.marginTop = '10px';

  // // Add click listener
  customButton.addEventListener('click', async () => {
    // send message to backend (complies with V3)
    chrome.runtime.sendMessage({ action: "fetchData",code:getCCode() }, (response) => {
      if (response.success) {
        console.log('Fetched data:', response.data);
      } else {
        console.error('Error fetching data:', response.error);
      }
    });
  });

  // // Insert our button after the Add to Cart button
  addToCartBtn.parentElement.insertBefore(customButton, addToCartBtn.nextSibling);
}

// Start the process when the page loads
document.addEventListener('DOMContentLoaded', addCustomButton);

// Also run on dynamic page updates (for single-page applications)
const observer = new MutationObserver(() => {
  if (!document.querySelector('button.add-2-project-tag')) {
    console.log("Adding Custom Button")
      addCustomButton();
  }
});

observer.observe(document.body, {
  childList: true,
  subtree: true
});