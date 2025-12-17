// JavaScript bindings for Agent Issuance WASM module
// This file demonstrates how to use the WASM module in a web application

let wasmModule = null;
let agent = null;

// Initialize the WASM module
async function initWasm() {
    try {
        showStatus('Loading WASM module...', 'info');

        // Import the WASM module
        // The path should point to the generated pkg directory
        const module = await import('../pkg/agent_issuance.js');

        // Initialize the module
        await module.default();

        wasmModule = module;

        // Display version
        const version = module.version();
        document.getElementById('version').textContent = `Version ${version}`;

        showStatus('WASM module loaded successfully!', 'success');

        // Enable the initialize button
        document.getElementById('initBtn').disabled = false;

        return true;
    } catch (error) {
        console.error('Failed to load WASM module:', error);
        showStatus(`Error loading WASM module: ${error.message}`, 'error');
        return false;
    }
}

// Initialize the agent with configuration
async function initializeAgent() {
    try {
        const issuerUrl = document.getElementById('issuerUrl').value;
        const issuerName = document.getElementById('issuerName').value;

        if (!issuerUrl || !issuerName) {
            showStatus('Please provide both issuer URL and name', 'error');
            return;
        }

        showStatus('Initializing agent...', 'info');

        // Create configuration
        const config = new wasmModule.WasmIssuanceConfig(issuerUrl, issuerName);

        // Create the agent
        agent = new wasmModule.WasmIssuanceAgent(config);

        showStatus('Agent initialized successfully!', 'success');

        // Enable action buttons
        document.getElementById('createOfferBtn').disabled = false;
        document.getElementById('issueBtn').disabled = false;
        document.getElementById('initBtn').textContent = 'Re-initialize Agent';

        console.log('Agent initialized:', agent);
    } catch (error) {
        console.error('Failed to initialize agent:', error);
        showStatus(`Error: ${error.message || error}`, 'error');
    }
}

// Create a credential offer
async function createCredentialOffer() {
    if (!agent) {
        showStatus('Please initialize the agent first', 'error');
        return;
    }

    try {
        showStatus('Creating credential offer...', 'info');

        // Parse credential configuration IDs
        const configIdsInput = document.getElementById('credentialConfigIds').value;
        const configIds = configIdsInput.split(',').map(id => id.trim());

        // Get grant types
        const grantTypeSelect = document.getElementById('grantTypes').value;
        let grantTypes = [];

        switch (grantTypeSelect) {
            case 'pre-authorized':
                grantTypes = ['urn:ietf:params:oauth:grant-type:pre-authorized_code'];
                break;
            case 'authorization':
                grantTypes = ['authorization_code'];
                break;
            case 'both':
                grantTypes = [
                    'urn:ietf:params:oauth:grant-type:pre-authorized_code',
                    'authorization_code'
                ];
                break;
        }

        // Create the offer
        const offerJson = await agent.create_credential_offer(configIds, grantTypes);

        // Parse and display the offer
        const offer = JSON.parse(offerJson);

        document.getElementById('offerOutput').style.display = 'block';
        document.getElementById('offerOutput').querySelector('pre').textContent =
            JSON.stringify(offer, null, 2);

        showStatus('Credential offer created successfully!', 'success');

        console.log('Credential offer:', offer);
    } catch (error) {
        console.error('Failed to create credential offer:', error);
        showStatus(`Error: ${error.message || error}`, 'error');
    }
}

// Issue a credential
async function issueCredential() {
    if (!agent) {
        showStatus('Please initialize the agent first', 'error');
        return;
    }

    try {
        showStatus('Issuing credential...', 'info');

        // Get credential data
        const credentialDataInput = document.getElementById('credentialData').value;
        const configId = document.getElementById('configId').value;

        // Validate JSON
        let credentialData;
        try {
            credentialData = JSON.parse(credentialDataInput);
        } catch (e) {
            throw new Error('Invalid JSON in credential data: ' + e.message);
        }

        // Issue the credential
        const credentialJson = await agent.issue_credential(
            JSON.stringify(credentialData),
            configId
        );

        // Parse and display the credential
        const credential = JSON.parse(credentialJson);

        document.getElementById('credentialOutput').style.display = 'block';
        document.getElementById('credentialOutput').querySelector('pre').textContent =
            JSON.stringify(credential, null, 2);

        showStatus('Credential issued successfully!', 'success');

        console.log('Issued credential:', credential);
    } catch (error) {
        console.error('Failed to issue credential:', error);
        showStatus(`Error: ${error.message || error}`, 'error');
    }
}

// Utility function to show status messages
function showStatus(message, type) {
    const statusDiv = document.getElementById('status');
    statusDiv.textContent = message;
    statusDiv.className = `status ${type}`;
    statusDiv.style.display = 'block';

    // Auto-hide info messages after 5 seconds
    if (type === 'info' || type === 'success') {
        setTimeout(() => {
            statusDiv.style.display = 'none';
        }, 5000);
    }
}

// Make functions globally available
window.initializeAgent = initializeAgent;
window.createCredentialOffer = createCredentialOffer;
window.issueCredential = issueCredential;

// Initialize WASM on page load
window.addEventListener('DOMContentLoaded', async () => {
    console.log('Agent Issuance WASM Demo');
    console.log('Initializing...');

    const success = await initWasm();

    if (!success) {
        console.error('Failed to initialize WASM module');
        showStatus(
            'Failed to load WASM module. Make sure to build the project first with ./build-wasm.sh',
            'error'
        );
    }
});

// Log any unhandled errors
window.addEventListener('error', (event) => {
    console.error('Unhandled error:', event.error);
    showStatus(`Unhandled error: ${event.error.message}`, 'error');
});

window.addEventListener('unhandledrejection', (event) => {
    console.error('Unhandled promise rejection:', event.reason);
    showStatus(`Unhandled error: ${event.reason}`, 'error');
});
