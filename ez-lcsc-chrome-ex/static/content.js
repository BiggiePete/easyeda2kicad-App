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
function sendMessage(action, message = undefined) {
	return new Promise((resolve, reject) => {
		chrome.runtime.sendMessage({ action: action, data: message }, (response) => {
			if (response) {
				resolve(response);
			} else {
				reject(new Error(response?.error || 'Unknown error occurred'));
			}
		});
	});
}
// structure
// [
// 	{
// 			"id": 1,
// 			"title": "testKiCADProject",
// 			"dir": "F:\\Desktop\\Projects\\testKiCADProject"
// 	}
// ]

let ifPending = false;
// Also run on dynamic page updates (for single-page applications)
// TODO, figure out why data isnt being read in properly
// TODO, finish making the table
const observer = new MutationObserver(async () => {
	if (!document.querySelector('div.ballerTable') && !ifPending) {
		ifPending = true;
		console.log('Adding Project Table');
		const projects = await sendMessage('getProjectList');
		console.log(projects);

		// Create the wrapper div with the specified styles
		const wrapperDiv = document.createElement('div');
		wrapperDiv.className = 'rounded white pa-6 mt-5';
		wrapperDiv.style.borderRadius = '4px';
		wrapperDiv.style.backgroundColor = 'white';
		wrapperDiv.style.padding = '24px';
		wrapperDiv.style.marginTop = '20px';

		// Get the target container
		const targetContainer = getTablePositionElement();

		// Insert our wrapper as the second child if possible
		if (targetContainer.childNodes.length > 0) {
			targetContainer.insertBefore(wrapperDiv, targetContainer.childNodes[1]);
		} else {
			targetContainer.appendChild(wrapperDiv);
		}

		// Add the button to our wrapper div instead of directly to the target
		appendCreateProjectButton('Add a project', wrapperDiv).addEventListener('click', createproject);

		if (projects.length == 0) {
			console.log('No Projects!');
			return;
		}

		// Create the table in our wrapper div instead of directly in the target
		createProjectTable(
			wrapperDiv,
			[
				{ text: 'Title', style: '' },
				{ text: '', style: '' }
			],
			projects.map((v) => {
				return {
					id: v.id,
					description: v.proj_name
				};
			})
		);
	}
});

observer.observe(document.body, {
	childList: true,
	subtree: true
});

// the goal is now, to add this element to the page somewhere
// TODO make the buttons fire events to the background
// TODO attach ids to the buttons, and make them talk correctly
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

		// Description column
		const tdDesc = document.createElement('td');
		tdDesc.style = 'line-height: 24px;';
		tdDesc.innerHTML = rowData.description;

		// Button column
		const tdButton = document.createElement('td');
		const buttonHtml = `
				<div class="flex flex-row-reverse" style="display:flex">
          <button id="EZLCSC_${rowData.id}" type="button" class="v-btn v-btn--is-elevated v-btn--has-bg theme--light v-size--small primary" style="height:32px;">
              <span class="v-btn__content">
                  <span class="font-Bold-600">Add To Project</span>
              </span>
          </button>
				</div>
      `;
		tdButton.innerHTML = buttonHtml;
		tr.appendChild(tdDesc);
		tr.appendChild(tdButton);
		tbody.appendChild(tr);
		tdButton.addEventListener('click', () => add2Project(rowData.id));
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
async function add2Project(id) {
	console.log('GOT ID: ' + id);
	const cCode = getCCode();
	const status = await sendMessage('add2Project', { c: cCode, id: id });
	console.log(status);
}
async function createproject() {
	const status = await sendMessage('createNewProject');
	console.log(status);
}

function getTablePositionElement() {
	return document.querySelector(
		'#app > div.v-application--wrap > main > div > div > div > div > div.flex-auto'
	);
}

function appendCreateProjectButton(text, targetElement, options = {}) {
	// Default options
	const { height = 42, backgroundColor = null } = options;

	// Create button element
	const button = document.createElement('button');

	// Set attributes
	button.setAttribute('type', 'button');
	button.setAttribute(
		'class',
		'v-btn v-btn--block v-btn--is-elevated v-btn--has-bg theme--light v-size--default secondary mt-2'
	);
	button.setAttribute('EZLCSC2KICAD-CREATEPROJECT', '');

	// Set style
	button.style.height = `${height}px`;
	if (backgroundColor) {
		button.style.backgroundColor = backgroundColor;
	}

	// Create span elements
	const contentSpan = document.createElement('span');
	contentSpan.setAttribute('class', 'v-btn__content');

	const textSpan = document.createElement('span');
	textSpan.setAttribute('class', 'font-Bold-600');
	textSpan.setAttribute('data-v-8acf6358', '');
	textSpan.textContent = text;

	// Build the button structure
	contentSpan.appendChild(textSpan);
	button.appendChild(contentSpan);

	// Find target element if string selector was provided
	let target;
	if (typeof targetElement === 'string') {
		target = document.querySelector(targetElement);
		if (!target) {
			console.error(`Target element "${targetElement}" not found`);
			return button; // Return button even if not appended
		}
	} else {
		target = targetElement;
	}

	// Append the button to the target
	target.appendChild(button);
	let span = document.createElement('span');
	span.textContent = 'If you have just created a project, please refresh the page';
	target.appendChild(span);

	return button;
}
