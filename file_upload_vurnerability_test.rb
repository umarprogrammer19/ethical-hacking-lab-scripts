#!/usr/bin/env ruby
# Checks for common file upload acceptance behavior by trying different
# Content-Type values for file parts.
#
# WARNING: Use this script only against systems you own or have explicit authorization to test.

require 'net/http'
require 'uri'
require 'securerandom'
require 'pathname'

def build_multipart_body(fields, file_field_name, filename, file_content, file_content_type, boundary)
  crlf = "\r\n"
  body = ''

  fields.each do |k, v|
    body << "--#{boundary}#{crlf}"
    body << %(Content-Disposition: form-data; name="#{k}"#{crlf}#{crlf})
    body << v.to_s << crlf
  end

  body << "--#{boundary}#{crlf}"
  body << %(Content-Disposition: form-data; name="#{file_field_name}"; filename="#{File.basename(filename)}"#{crlf})
  body << "Content-Type: #{file_content_type}#{crlf}#{crlf}"
  body << file_content
  body << crlf
  body << "--#{boundary}--#{crlf}"

  body
end

def test_upload(target_url, filename)
  uri = URI.parse(target_url)

  begin
    file_content = File.binread(filename)
  rescue => e
    puts "Error reading #{filename}: #{e}"
    return
  end

  content_types = [
    'application/octet-stream',
    'text/plain',
    'image/png'
  ]

  fields = { 'submit' => 'Upload' }

  content_types.each do |ct|
    boundary = "----RubyMultipart#{SecureRandom.hex(12)}"
    body = build_multipart_body(fields, 'file', filename, file_content, ct, boundary)

    req = Net::HTTP::Post.new(uri.request_uri)
    req['Content-Type'] = "multipart/form-data; boundary=#{boundary}"
    req.body = body

    puts "Trying: #{filename}, Content-Type (file part): #{ct}"

    http = Net::HTTP.new(uri.host, uri.port)
    if uri.scheme == 'https'
      http.use_ssl = true
      http.verify_mode = OpenSSL::SSL::VERIFY_NONE
    end
    http.read_timeout = 10
    http.open_timeout = 5

    begin
      resp = http.start { |h| h.request(req) }

      body_text = resp.body.to_s.downcase
      is_redirect = resp.is_a?(Net::HTTPRedirection) || resp.code.to_s.start_with?('3')

      if body_text.include?('success') || is_redirect || resp.code.to_i == 200 && body_text.length < 200 && body_text.include?('uploaded')
        puts "[VULNERABLE] Accepts #{ct} as valid upload (status #{resp.code})"
      else
        puts "Not accepted (status #{resp.code})"
      end

    rescue => e
      puts "Request failed for content-type #{ct}: #{e}"
    end
  end
end

def main
  if ARGV.length != 2
    puts "Usage: #{$0} <upload_url> <file_glob_pattern>"
    puts "Example: #{$0} https://example.com/upload '*.png'"
    exit 1
  end

  url = ARGV[0]
  pattern = ARGV[1]

  files = Dir.glob(pattern)
  if files.empty?
    puts "No files matched pattern: #{pattern}"
    exit 1
  end

  puts "Starting tests on #{url}, files: #{pattern}"

  files.each do |f|
    test_upload(url, f)
  end
end

if __FILE__ == $0
  main
end
