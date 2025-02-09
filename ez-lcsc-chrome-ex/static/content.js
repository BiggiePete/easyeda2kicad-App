function findAddToCartButton() {
	return document.querySelector('div.mt-5[data-v-8acf6358] button.v-btn.primary[data-v-8acf6358]');
}

function getCCode() {
	function findAllCSharps() {
		// Find all the spans with class 'major2--text'
		const cSharpSpans = document.querySelectorAll('span.major2--text');

		// Map through the NodeList and extract text content from each span
		const cSharpValues = Array.from(cSharpSpans).map((span) => span.textContent.trim());

		return cSharpValues;
	}
	return findAllCSharps().filter((v) => {
		return v.charAt(0) === 'C' && !isNaN(v.slice(1)) && v.slice(1).length > 0;
	})[0];
}

// Function to add our custom button next to the Add to Cart button
function addCustomButton() {
	const addToCartBtn = findAddToCartButton();
	if (!addToCartBtn) {
		console.log('Target button not found, retrying...');
		setTimeout(addCustomButton, 1000); // Retry after 1 second
	}
	// // Create our custom button with similar styling
	const customButton = document.createElement('button');
	customButton.textContent = 'Add To Project';
	customButton.className =
		'v-btn v-btn--block v-btn--is-elevated v-btn--has-bg theme--light v-size--default secondary mt-5 add-2-project-tag';
	customButton.style.height = '42px';
	customButton.style.marginTop = '10px';

	// // Add click listener
	customButton.addEventListener('click', async () => {
		// send message to backend (complies with V3)
		chrome.runtime.sendMessage({ action: 'openPOPUP' }, (response) => {
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
	if (!document.querySelector('div.ballerTable')) {
		console.log('Adding Custom Button');
		// addCustomButton();
		createProjectTable(
			getTablePositionElement(),
			[
				{ text: 'h1', style: '' },
				{ text: 'h2', style: '' },
				{ text: 'h3', style: '' }
			],
			[
				{ type: '1', description: 'something1' },
				{ type: '2', description: 'something2' },
				{ type: '3', description: 'something3' }
			]
		);
	}
});

observer.observe(document.body, {
	childList: true,
	subtree: true
});

// the goal is now, to add this element to the page somewhere

function createProjectTable(targetElement, tableHeaders, tableData) {
	// Create main container
	const container = document.createElement('div');
	container.className = 'v-data-table common-table-v7 mt-3 theme--light ballerTable';

	// Create wrapper
	const wrapper = document.createElement('div');
	wrapper.className = 'v-data-table__wrapper';

	// Create table
	const table = document.createElement('table');

	// Create thead
	const thead = document.createElement('thead');
	const headerRow = document.createElement('tr');

	// Create header cells
	const headers = tableHeaders;

	headers.forEach((header) => {
		const th = document.createElement('th');
		if (header.style) th.style = header.style;
		th.textContent = header.text;
		headerRow.appendChild(th);
	});

	thead.appendChild(headerRow);

	// Create tbody
	const tbody = document.createElement('tbody');

	// Add sample data rows
	const rows = tableData;

	rows.forEach((rowData, index) => {
		const tr = document.createElement('tr');

		// Type column
		const tdType = document.createElement('td');
		tdType.id = `${rowData.type.toLowerCase()}_id`;
		tdType.textContent = rowData.type;

		// Description column
		const tdDesc = document.createElement('td');
		tdDesc.style = 'line-height: 24px;';
		tdDesc.innerHTML = rowData.description;

		// Button column
		const tdButton = document.createElement('td');
		const buttonHtml = `
          <button type="button" class="v-btn v-btn--is-elevated v-btn--has-bg theme--light v-size--small primary" style="height:32px;">
              <span class="v-btn__content">
                  <span class="font-Bold-600">Add To Project</span>
              </span>
          </button>
      `;
		tdButton.innerHTML = buttonHtml;

		tr.appendChild(tdType);
		tr.appendChild(tdDesc);
		tr.appendChild(tdButton);
		tbody.appendChild(tr);
	});

	// Assemble the table
	table.appendChild(thead);
	table.appendChild(tbody);
	wrapper.appendChild(table);
	container.appendChild(wrapper);

	// Add styles
	const style = document.createElement('style');
	style.textContent = `
      .v-data-table {
          border: 1px solid rgba(0,0,0,0.12);
          border-radius: 4px;
          background-color: #fff;
      }
      .v-data-table__wrapper {
          overflow-x: auto;
      }
      .theme--light {
          color: rgba(0,0,0,0.87);
      }
      .mt-3 {
          margin-top: 12px;
      }
      .v2-a {
          color: #0969da;
          text-decoration: none;
      }
      td, th {
          padding: 0 16px;
          height: 48px;
      }
      .v-btn {
          align-items: center;
          border-radius: 4px;
          display: inline-flex;
          flex: 0 0 auto;
          font-weight: 500;
          letter-spacing: .0892857143em;
          justify-content: center;
          outline: 0;
          position: relative;
          text-decoration: none;
          text-indent: .0892857143em;
          text-transform: uppercase;
          transition-duration: .28s;
          transition-property: box-shadow,transform,opacity;
          transition-timing-function: cubic-bezier(.4,0,.2,1);
          user-select: none;
          vertical-align: middle;
          white-space: nowrap;
      }
      .v-btn--is-elevated {
          box-shadow: 0 3px 1px -2px rgba(0,0,0,.2),0 2px 2px 0 rgba(0,0,0,.14),0 1px 5px 0 rgba(0,0,0,.12);
      }
      .v-btn--has-bg {
          background-color: #1976d2;
      }
      .v-size--small {
          font-size: .875rem;
      }
      .primary {
          background-color: #1976d2 !important;
          border-color: #1976d2 !important;
          color: #fff !important;
      }
      .v-btn__content {
          align-items: center;
          color: inherit;
          display: flex;
          flex: 1 0 auto;
          justify-content: inherit;
          line-height: normal;
          position: relative;
      }
      .font-Bold-600 {
          font-weight: 600;
      }
  `;
	document.head.appendChild(style);

	// Insert into target element
	if (targetElement) {
		targetElement.appendChild(container);
	}
}

function getTablePositionElement() {
	return document.querySelector(
		'#app > div.v-application--wrap > main > div > div > div > div > div.ml-lg-5.mt-5.mt-lg-0.detailRightWrap > div.rounded.white.pa-6'
	);
}
