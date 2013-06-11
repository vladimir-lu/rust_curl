macro_rules! CINIT(
    ($name:ident, $typ:expr, $val:expr) => (
        pub static $name: i32 = $typ + $val as i32;
    )
)


static LONG: i32 = 0i32;
static OBJECTPOINT: i32 = 10_000i32;
static FUNCTIONPOINT: i32 = 20_000i32;
static OFF_T: i32 = 30_000i32;

 /* This is the FILE * or void * the regular output should be written to. */
  CINIT!(FILE, OBJECTPOINT, 1)
  
  pub static WRITEDATA: i32 = FILE;

  /* The full URL to get/put */
  CINIT!(URL,  OBJECTPOINT, 2)

  /* Port number to connect to, if other than default. */
  CINIT!(PORT, LONG, 3)

  /* Name of proxy to use. */
  CINIT!(PROXY, OBJECTPOINT, 4)

  /* "name:password" to use when fetching. */
  CINIT!(USERPWD, OBJECTPOINT, 5)

  /* "name:password" to use with proxy. */
  CINIT!(PROXYUSERPWD, OBJECTPOINT, 6)

  /* Range to get, specified as an ASCII string. */
  CINIT!(RANGE, OBJECTPOINT, 7)

  /* not used */

  /* Specified file stream to upload from (use as input): */
  CINIT!(INFILE, OBJECTPOINT, 9)

  /* Buffer to receive error messages in, must be at least CURL_ERROR_SIZE
   * bytes big. If this is not used, error messages go to stderr instead: */
  CINIT!(ERRORBUFFER, OBJECTPOINT, 10)

  /* Function that will be called to store the output (instead of fwrite). The
   * parameters will use fwrite() syntax, make sure to follow them. */
  CINIT!(WRITEFUNCTION, FUNCTIONPOINT, 11)

  /* Function that will be called to read the input (instead of fread). The
   * parameters will use fread() syntax, make sure to follow them. */
  CINIT!(READFUNCTION, FUNCTIONPOINT, 12)

  /* Time-out the read operation after this amount of seconds */
  CINIT!(TIMEOUT, LONG, 13)

  /* If the CURLOPT_INFILE is used, this can be used to inform libcurl about
   * how large the file being sent really is. That allows better error
   * checking and better verifies that the upload was successful. -1 means
   * unknown size.
   *
   * For large file support, there is also a _LARGE version of the key
   * which takes an off_t type, allowing platforms with larger off_t
   * sizes to handle larger files.  See below for INFILESIZE_LARGE.
   */
  CINIT!(INFILESIZE, LONG, 14)

  /* POST static input fields. */
  CINIT!(POSTFIELDS, OBJECTPOINT, 15)

  /* Set the referrer page (needed by some CGIs) */
  CINIT!(REFERER, OBJECTPOINT, 16)

  /* Set the FTP PORT string (interface name, named or numerical IP address)
     Use i.e '-' to use default address. */
  CINIT!(FTPPORT, OBJECTPOINT, 17)

  /* Set the User-Agent string (examined by some CGIs) */
  CINIT!(USERAGENT, OBJECTPOINT, 18)

  /* If the download receives less than "low speed limit" bytes/second
   * during "low speed time" seconds, the operations is aborted.
   * You could i.e if you have a pretty high speed connection, abort if
   * it is less than 2000 bytes/sec during 20 seconds.
   */

  /* Set the "low speed limit" */
  CINIT!(LOW_SPEED_LIMIT, LONG, 19)

  /* Set the "low speed time" */
  CINIT!(LOW_SPEED_TIME, LONG, 20)

  /* Set the continuation offset.
   *
   * Note there is also a _LARGE version of this key which uses
   * off_t types, allowing for large file offsets on platforms which
   * use larger-than-32-bit off_t's.  Look below for RESUME_FROM_LARGE.
   */
  CINIT!(RESUME_FROM, LONG, 21)

  /* Set cookie in request: */
  CINIT!(COOKIE, OBJECTPOINT, 22)

  /* This points to a linked list of headers, struct curl_slist kind */
  CINIT!(HTTPHEADER, OBJECTPOINT, 23)

  /* This points to a linked list of post entries, struct curl_httppost */
  CINIT!(HTTPPOST, OBJECTPOINT, 24)

  /* name of the file keeping your private SSL-certificate */
  CINIT!(SSLCERT, OBJECTPOINT, 25)

  /* password for the SSL or SSH private key */
  CINIT!(KEYPASSWD, OBJECTPOINT, 26)

  /* send TYPE parameter? */
  CINIT!(CRLF, LONG, 27)

  /* send linked-list of QUOTE commands */
  CINIT!(QUOTE, OBJECTPOINT, 28)

  /* send FILE * or void * to store headers to, if you use a callback it
     is simply passed to the callback unmodified */
  CINIT!(WRITEHEADER, OBJECTPOINT, 29)
  
  pub static HEADERDATA: i32 = WRITEHEADER;

  /* point to a file to read the initial cookies from, also enables
     "cookie awareness" */
  CINIT!(COOKIEFILE, OBJECTPOINT, 31)

  /* What version to specifically try to use.
     See CURL_SSLVERSION defines below. */
  CINIT!(SSLVERSION, LONG, 32)

  /* What kind of HTTP time condition to use, see defines */
  CINIT!(TIMECONDITION, LONG, 33)

  /* Time to use with the above condition. Specified in number of seconds
     since 1 Jan 1970 */
  CINIT!(TIMEVALUE, LONG, 34)

  /* 35 = OBSOLETE */

  /* Custom request, for customizing the get command like
     HTTP: DELETE, TRACE and others
     FTP: to use a different list command
     */
  CINIT!(CUSTOMREQUEST, OBJECTPOINT, 36)

  /* HTTP request, for odd commands like DELETE, TRACE and others */
  CINIT!(STDERR, OBJECTPOINT, 37)

  /* 38 is not used */

  /* send linked-list of post-transfer QUOTE commands */
  CINIT!(POSTQUOTE, OBJECTPOINT, 39)

  CINIT!(WRITEINFO, OBJECTPOINT, 40) /* DEPRECATED, do not use! */

  CINIT!(VERBOSE, LONG, 41)      /* talk a lot */
  CINIT!(HEADER, LONG, 42)       /* throw the header out too */
  CINIT!(NOPROGRESS, LONG, 43)   /* shut off the progress meter */
  CINIT!(NOBODY, LONG, 44)       /* use HEAD to get http document */
  CINIT!(FAILONERROR, LONG, 45)  /* no output on http error codes >= 300 */
  CINIT!(UPLOAD, LONG, 46)       /* this is an upload */
  CINIT!(POST, LONG, 47)         /* HTTP POST method */
  CINIT!(DIRLISTONLY, LONG, 48)  /* bare names when listing directories */

  CINIT!(APPEND, LONG, 50)       /* Append instead of overwrite on upload! */

  /* Specify whether to read the user+password from the .netrc or the URL.
   * This must be one of the CURL_NETRC_* enums below. */
  CINIT!(NETRC, LONG, 51)

  CINIT!(FOLLOWLOCATION, LONG, 52)  /* use Location: Luke! */

  CINIT!(TRANSFERTEXT, LONG, 53) /* transfer data in text/ASCII format */
  CINIT!(PUT, LONG, 54)          /* HTTP PUT */

  /* 55 = OBSOLETE */

  /* Function that will be called instead of the internal progress display
   * function. This function should be defined as the curl_progress_callback
   * prototype defines. */
  CINIT!(PROGRESSFUNCTION, FUNCTIONPOINT, 56)

  /* Data passed to the progress callback */
  CINIT!(PROGRESSDATA, OBJECTPOINT, 57)

  /* We want the referrer field set automatically when following locations */
  CINIT!(AUTOREFERER, LONG, 58)

  /* Port of the proxy, can be set in the proxy string as well with:
     "[host]:[port]" */
  CINIT!(PROXYPORT, LONG, 59)

  /* size of the POST input data, if strlen() is not good to use */
  CINIT!(POSTFIELDSIZE, LONG, 60)

  /* tunnel non-http operations through a HTTP proxy */
  CINIT!(HTTPPROXYTUNNEL, LONG, 61)

  /* Set the interface string to use as outgoing network interface */
  CINIT!(INTERFACE, OBJECTPOINT, 62)

  /* Set the krb4/5 security level, this also enables krb4/5 awareness.  This
   * is a string, 'clear', 'safe', 'confidential' or 'private'.  If the string
   * is set but doesn't match one of these, 'private' will be used.  */
  CINIT!(KRBLEVEL, OBJECTPOINT, 63)

  /* Set if we should verify the peer in ssl handshake, set 1 to verify. */
  CINIT!(SSL_VERIFYPEER, LONG, 64)

  /* The CApath or CAfile used to validate the peer certificate
     this option is used only if SSL_VERIFYPEER is true */
  CINIT!(CAINFO, OBJECTPOINT, 65)

  /* 66 = OBSOLETE */
  /* 67 = OBSOLETE */

  /* Maximum number of http redirects to follow */
  CINIT!(MAXREDIRS, LONG, 68)

  /* Pass a long set to 1 to get the date of the requested document (if
     possible)! Pass a zero to shut it off. */
  CINIT!(FILETIME, LONG, 69)

  /* This points to a linked list of telnet options */
  CINIT!(TELNETOPTIONS, OBJECTPOINT, 70)

  /* Max amount of cached alive connections */
  CINIT!(MAXCONNECTS, LONG, 71)

  CINIT!(CLOSEPOLICY, LONG, 72) /* DEPRECATED, do not use! */

  /* 73 = OBSOLETE */

  /* Set to explicitly use a new connection for the upcoming transfer.
     Do not use this unless you're absolutely sure of this, as it makes the
     operation slower and is less friendly for the network. */
  CINIT!(FRESH_CONNECT, LONG, 74)

  /* Set to explicitly forbid the upcoming transfer's connection to be re-used
     when done. Do not use this unless you're absolutely sure of this, as it
     makes the operation slower and is less friendly for the network. */
  CINIT!(FORBID_REUSE, LONG, 75)

  /* Set to a file name that contains random data for libcurl to use to
     seed the random engine when doing SSL connects. */
  CINIT!(RANDOM_FILE, OBJECTPOINT, 76)

  /* Set to the Entropy Gathering Daemon socket pathname */
  CINIT!(EGDSOCKET, OBJECTPOINT, 77)

  /* Time-out connect operations after this amount of seconds, if connects are
     OK within this time, then fine... This only aborts the connect phase. */
  CINIT!(CONNECTTIMEOUT, LONG, 78)

  /* Function that will be called to store headers (instead of fwrite). The
   * parameters will use fwrite() syntax, make sure to follow them. */
  CINIT!(HEADERFUNCTION, FUNCTIONPOINT, 79)

  /* Set this to force the HTTP request to get back to GET. Only really usable
     if POST, PUT or a custom request have been used first.
   */
  CINIT!(HTTPGET, LONG, 80)

  /* Set if we should verify the Common name from the peer certificate in ssl
   * handshake, set 1 to check existence, 2 to ensure that it matches the
   * provided hostname. */
  CINIT!(SSL_VERIFYHOST, LONG, 81)

  /* Specify which file name to write all known cookies in after completed
     operation. Set file name to "-" (dash) to make it go to stdout. */
  CINIT!(COOKIEJAR, OBJECTPOINT, 82)

  /* Specify which SSL ciphers to use */
  CINIT!(SSL_CIPHER_LIST, OBJECTPOINT, 83)

  /* Specify which HTTP version to use! This must be set to one of the
     CURL_HTTP_VERSION* enums set below. */
  CINIT!(HTTP_VERSION, LONG, 84)

  /* Specifically switch on or off the FTP engine's use of the EPSV command. By
     default, that one will always be attempted before the more traditional
     PASV command. */
  CINIT!(FTP_USE_EPSV, LONG, 85)

  /* type of the file keeping your SSL-certificate ("DER", "PEM", "ENG") */
  CINIT!(SSLCERTTYPE, OBJECTPOINT, 86)

  /* name of the file keeping your private SSL-key */
  CINIT!(SSLKEY, OBJECTPOINT, 87)

  /* type of the file keeping your private SSL-key ("DER", "PEM", "ENG") */
  CINIT!(SSLKEYTYPE, OBJECTPOINT, 88)

  /* crypto engine for the SSL-sub system */
  CINIT!(SSLENGINE, OBJECTPOINT, 89)

  /* set the crypto engine for the SSL-sub system as default
     the param has no meaning...
   */
  CINIT!(SSLENGINE_DEFAULT, LONG, 90)

  /* Non-zero value means to use the global dns cache */
  CINIT!(DNS_USE_GLOBAL_CACHE, LONG, 91) /* DEPRECATED, do not use! */

  /* DNS cache timeout */
  CINIT!(DNS_CACHE_TIMEOUT, LONG, 92)

  /* send linked-list of pre-transfer QUOTE commands */
  CINIT!(PREQUOTE, OBJECTPOINT, 93)

  /* set the debug function */
  CINIT!(DEBUGFUNCTION, FUNCTIONPOINT, 94)

  /* set the data for the debug function */
  CINIT!(DEBUGDATA, OBJECTPOINT, 95)

  /* mark this as start of a cookie session */
  CINIT!(COOKIESESSION, LONG, 96)

  /* The CApath directory used to validate the peer certificate
     this option is used only if SSL_VERIFYPEER is true */
  CINIT!(CAPATH, OBJECTPOINT, 97)

  /* Instruct libcurl to use a smaller receive buffer */
  CINIT!(BUFFERSIZE, LONG, 98)

  /* Instruct libcurl to not use any signal/alarm handlers, even when using
     timeouts. This option is useful for multi-threaded applications.
     See libcurl-the-guide for more background information. */
  CINIT!(NOSIGNAL, LONG, 99)

  /* Provide a CURLShare for mutexing non-ts data */
  CINIT!(SHARE, OBJECTPOINT, 100)

  /* indicates type of proxy. accepted values are CURLPROXY_HTTP (default)
     CURLPROXY_SOCKS4, CURLPROXY_SOCKS4A and CURLPROXY_SOCKS5. */
  CINIT!(PROXYTYPE, LONG, 101)

  /* Set the Accept-Encoding string. Use this to tell a server you would like
     the response to be compressed. Before 7.21.6, this was known as
     CURLOPT_ENCODING */
  CINIT!(ACCEPT_ENCODING, OBJECTPOINT, 102)

  /* Set pointer to private data */
  CINIT!(PRIVATE, OBJECTPOINT, 103)

  /* Set aliases for HTTP 200 in the HTTP Response header */
  CINIT!(HTTP200ALIASES, OBJECTPOINT, 104)

  /* Continue to send authentication (user+password) when following locations,
     even when hostname changed. This can potentially send off the name
     and password to whatever host the server decides. */
  CINIT!(UNRESTRICTED_AUTH, LONG, 105)

  /* Specifically switch on or off the FTP engine's use of the EPRT command (
     it also disables the LPRT attempt). By default, those ones will always be
     attempted before the good old traditional PORT command. */
  CINIT!(FTP_USE_EPRT, LONG, 106)

  /* Set this to a bitmask value to enable the particular authentications
     methods you like. Use this in combination with CURLOPT_USERPWD.
     Note that setting multiple bits may cause extra network round-trips. */
  CINIT!(HTTPAUTH, LONG, 107)

  /* Set the ssl context callback function, currently only for OpenSSL ssl_ctx
     in second argument. The function must be matching the
     curl_ssl_ctx_callback proto. */
  CINIT!(SSL_CTX_FUNCTION, FUNCTIONPOINT, 108)

  /* Set the userdata for the ssl context callback function's third
     argument */
  CINIT!(SSL_CTX_DATA, OBJECTPOINT, 109)

  /* FTP Option that causes missing dirs to be created on the remote server.
     In 7.19.4 we introduced the convenience enums for this option using the
     CURLFTP_CREATE_DIR prefix.
  */
  CINIT!(FTP_CREATE_MISSING_DIRS, LONG, 110)

  /* Set this to a bitmask value to enable the particular authentications
     methods you like. Use this in combination with CURLOPT_PROXYUSERPWD.
     Note that setting multiple bits may cause extra network round-trips. */
  CINIT!(PROXYAUTH, LONG, 111)

  /* FTP option that changes the timeout, in seconds, associated with
     getting a response.  This is different from transfer timeout time and
     essentially places a demand on the FTP server to acknowledge commands
     in a timely manner. */
  CINIT!(FTP_RESPONSE_TIMEOUT, LONG, 112)
  
  pub static SERVER_RESPONSE_TIMEOUT: i32 = FTP_RESPONSE_TIMEOUT;

  /* Set this option to one of the CURL_IPRESOLVE_* defines (see below) to
     tell libcurl to resolve names to those IP versions only. This only has
     affect on systems with support for more than one, i.e IPv4 _and_ IPv6. */
  CINIT!(IPRESOLVE, LONG, 113)

  /* Set this option to limit the size of a file that will be downloaded from
     an HTTP or FTP server.

     Note there is also _LARGE version which adds large file support for
     platforms which have larger off_t sizes.  See MAXFILESIZE_LARGE below. */
  CINIT!(MAXFILESIZE, LONG, 114)

  /* See the comment for INFILESIZE above, but in short, specifies
   * the size of the file being uploaded.  -1 means unknown.
   */
  CINIT!(INFILESIZE_LARGE, OFF_T, 115)

  /* Sets the continuation offset.  There is also a LONG version of this;
   * look above for RESUME_FROM.
   */
  CINIT!(RESUME_FROM_LARGE, OFF_T, 116)

  /* Sets the maximum size of data that will be downloaded from
   * an HTTP or FTP server.  See MAXFILESIZE above for the LONG version.
   */
  CINIT!(MAXFILESIZE_LARGE, OFF_T, 117)

  /* Set this option to the file name of your .netrc file you want libcurl
     to parse (using the CURLOPT_NETRC option). If not set, libcurl will do
     a poor attempt to find the user's home directory and check for a .netrc
     file in there. */
  CINIT!(NETRC_FILE, OBJECTPOINT, 118)

  /* Enable SSL/TLS for FTP, pick one of:
     CURLUSESSL_TRY     - try using SSL, proceed anyway otherwise
     CURLUSESSL_CONTROL - SSL for the control connection or fail
     CURLUSESSL_ALL     - SSL for all communication or fail
  */
  CINIT!(USE_SSL, LONG, 119)

  /* The _LARGE version of the standard POSTFIELDSIZE option */
  CINIT!(POSTFIELDSIZE_LARGE, OFF_T, 120)

  /* Enable/disable the TCP Nagle algorithm */
  CINIT!(TCP_NODELAY, LONG, 121)

  /* 122 OBSOLETE, used in 7.12.3. Gone in 7.13.0 */
  /* 123 OBSOLETE. Gone in 7.16.0 */
  /* 124 OBSOLETE, used in 7.12.3. Gone in 7.13.0 */
  /* 125 OBSOLETE, used in 7.12.3. Gone in 7.13.0 */
  /* 126 OBSOLETE, used in 7.12.3. Gone in 7.13.0 */
  /* 127 OBSOLETE. Gone in 7.16.0 */
  /* 128 OBSOLETE. Gone in 7.16.0 */

  /* When FTP over SSL/TLS is selected (with CURLOPT_USE_SSL) this option
     can be used to change libcurl's default action which is to first try
     "AUTH SSL" and then "AUTH TLS" in this order, and proceed when a OK
     response has been received.

     Available parameters are:
     CURLFTPAUTH_DEFAULT - let libcurl decide
     CURLFTPAUTH_SSL     - try "AUTH SSL" first, then TLS
     CURLFTPAUTH_TLS     - try "AUTH TLS" first, then SSL
  */
  CINIT!(FTPSSLAUTH, LONG, 129)

  CINIT!(IOCTLFUNCTION, FUNCTIONPOINT, 130)
  CINIT!(IOCTLDATA, OBJECTPOINT, 131)

  /* 132 OBSOLETE. Gone in 7.16.0 */
  /* 133 OBSOLETE. Gone in 7.16.0 */

  /* zero terminated string for pass on to the FTP server when asked for
     "account" info */
  CINIT!(FTP_ACCOUNT, OBJECTPOINT, 134)

  /* feed cookies into cookie engine */
  CINIT!(COOKIELIST, OBJECTPOINT, 135)

  /* ignore Content-Length */
  CINIT!(IGNORE_CONTENT_LENGTH, LONG, 136)

  /* Set to non-zero to skip the IP address received in a 227 PASV FTP server
     response. Typically used for FTP-SSL purposes but is not restricted to
     that. libcurl will then instead use the same IP address it used for the
     control connection. */
  CINIT!(FTP_SKIP_PASV_IP, LONG, 137)

  /* Select "file method" to use when doing FTP, see the curl_ftpmethod
     above. */
  CINIT!(FTP_FILEMETHOD, LONG, 138)

  /* Local port number to bind the socket to */
  CINIT!(LOCALPORT, LONG, 139)

  /* Number of ports to try, including the first one set with LOCALPORT.
     Thus, setting it to 1 will make no additional attempts but the first.
  */
  CINIT!(LOCALPORTRANGE, LONG, 140)

  /* no transfer, set up connection and let application use the socket by
     extracting it with CURLINFO_LASTSOCKET */
  CINIT!(CONNECT_ONLY, LONG, 141)

  /* Function that will be called to convert from the
     network encoding (instead of using the iconv calls in libcurl) */
  CINIT!(CONV_FROM_NETWORK_FUNCTION, FUNCTIONPOINT, 142)

  /* Function that will be called to convert to the
     network encoding (instead of using the iconv calls in libcurl) */
  CINIT!(CONV_TO_NETWORK_FUNCTION, FUNCTIONPOINT, 143)

  /* Function that will be called to convert from UTF8
     (instead of using the iconv calls in libcurl)
     Note that this is used only for SSL certificate processing */
  CINIT!(CONV_FROM_UTF8_FUNCTION, FUNCTIONPOINT, 144)

  /* if the connection proceeds too quickly then need to slow it down */
  /* limit-rate: maximum number of bytes per second to send or receive */
  CINIT!(MAX_SEND_SPEED_LARGE, OFF_T, 145)
  CINIT!(MAX_RECV_SPEED_LARGE, OFF_T, 146)

  /* Pointer to command string to send if USER/PASS fails. */
  CINIT!(FTP_ALTERNATIVE_TO_USER, OBJECTPOINT, 147)

  /* callback function for setting socket options */
  CINIT!(SOCKOPTFUNCTION, FUNCTIONPOINT, 148)
  CINIT!(SOCKOPTDATA, OBJECTPOINT, 149)

  /* set to 0 to disable session ID re-use for this transfer, default is
     enabled (== 1) */
  CINIT!(SSL_SESSIONID_CACHE, LONG, 150)

  /* allowed SSH authentication methods */
  CINIT!(SSH_AUTH_TYPES, LONG, 151)

  /* Used by scp/sftp to do public/private key authentication */
  CINIT!(SSH_PUBLIC_KEYFILE, OBJECTPOINT, 152)
  CINIT!(SSH_PRIVATE_KEYFILE, OBJECTPOINT, 153)

  /* Send CCC (Clear Command Channel) after authentication */
  CINIT!(FTP_SSL_CCC, LONG, 154)

  /* Same as TIMEOUT and CONNECTTIMEOUT, but with ms resolution */
  CINIT!(TIMEOUT_MS, LONG, 155)
  CINIT!(CONNECTTIMEOUT_MS, LONG, 156)

  /* set to zero to disable the libcurl's decoding and thus pass the raw body
     data to the application even when it is encoded/compressed */
  CINIT!(HTTP_TRANSFER_DECODING, LONG, 157)
  CINIT!(HTTP_CONTENT_DECODING, LONG, 158)

  /* Permission used when creating new files and directories on the remote
     server for protocols that support it, SFTP/SCP/FILE */
  CINIT!(NEW_FILE_PERMS, LONG, 159)
  CINIT!(NEW_DIRECTORY_PERMS, LONG, 160)

  /* Set the behaviour of POST when redirecting. Values must be set to one
     of CURL_REDIR* defines below. This used to be called CURLOPT_POST301 */
  CINIT!(POSTREDIR, LONG, 161)

  /* used by scp/sftp to verify the host's public key */
  CINIT!(SSH_HOST_PUBLIC_KEY_MD5, OBJECTPOINT, 162)

  /* Callback function for opening socket (instead of socket(2)). Optionally,
     callback is able change the address or refuse to connect returning
     CURL_SOCKET_BAD.  The callback should have type
     curl_opensocket_callback */
  CINIT!(OPENSOCKETFUNCTION, FUNCTIONPOINT, 163)
  CINIT!(OPENSOCKETDATA, OBJECTPOINT, 164)

  /* POST volatile input fields. */
  CINIT!(COPYPOSTFIELDS, OBJECTPOINT, 165)

  /* set transfer mode (;type=<a|i>) when doing FTP via an HTTP proxy */
  CINIT!(PROXY_TRANSFER_MODE, LONG, 166)

  /* Callback function for seeking in the input stream */
  CINIT!(SEEKFUNCTION, FUNCTIONPOINT, 167)
  CINIT!(SEEKDATA, OBJECTPOINT, 168)

  /* CRL file */
  CINIT!(CRLFILE, OBJECTPOINT, 169)

  /* Issuer certificate */
  CINIT!(ISSUERCERT, OBJECTPOINT, 170)

  /* (IPv6) Address scope */
  CINIT!(ADDRESS_SCOPE, LONG, 171)

  /* Collect certificate chain info and allow it to get retrievable with
     CURLINFO_CERTINFO after the transfer is complete. (Unfortunately) only
     working with OpenSSL-powered builds. */
  CINIT!(CERTINFO, LONG, 172)

  /* "name" and "pwd" to use when fetching. */
  CINIT!(USERNAME, OBJECTPOINT, 173)
  CINIT!(PASSWORD, OBJECTPOINT, 174)

    /* "name" and "pwd" to use with Proxy when fetching. */
  CINIT!(PROXYUSERNAME, OBJECTPOINT, 175)
  CINIT!(PROXYPASSWORD, OBJECTPOINT, 176)

  /* Comma separated list of hostnames defining no-proxy zones. These should
     match both hostnames directly, and hostnames within a domain. For
     example, local.com will match local.com and www.local.com, but NOT
     notlocal.com or www.notlocal.com. For compatibility with other
     implementations of this, .local.com will be considered to be the same as
     local.com. A single * is the only valid wildcard, and effectively
     disables the use of proxy. */
  CINIT!(NOPROXY, OBJECTPOINT, 177)

  /* block size for TFTP transfers */
  CINIT!(TFTP_BLKSIZE, LONG, 178)

  /* Socks Service */
  CINIT!(SOCKS5_GSSAPI_SERVICE, OBJECTPOINT, 179)

  /* Socks Service */
  CINIT!(SOCKS5_GSSAPI_NEC, LONG, 180)

  /* set the bitmask for the protocols that are allowed to be used for the
     transfer, which thus helps the app which takes URLs from users or other
     external inputs and want to restrict what protocol(s) to deal
     with. Defaults to CURLPROTO_ALL. */
  CINIT!(PROTOCOLS, LONG, 181)

  /* set the bitmask for the protocols that libcurl is allowed to follow to,
     as a subset of the CURLOPT_PROTOCOLS ones. That means the protocol needs
     to be set in both bitmasks to be allowed to get redirected to. Defaults
     to all protocols except FILE and SCP. */
  CINIT!(REDIR_PROTOCOLS, LONG, 182)

  /* set the SSH knownhost file name to use */
  CINIT!(SSH_KNOWNHOSTS, OBJECTPOINT, 183)

  /* set the SSH host key callback, must point to a curl_sshkeycallback
     function */
  CINIT!(SSH_KEYFUNCTION, FUNCTIONPOINT, 184)

  /* set the SSH host key callback custom pointer */
  CINIT!(SSH_KEYDATA, OBJECTPOINT, 185)

  /* set the SMTP mail originator */
  CINIT!(MAIL_FROM, OBJECTPOINT, 186)

  /* set the SMTP mail receiver(s) */
  CINIT!(MAIL_RCPT, OBJECTPOINT, 187)

  /* FTP: send PRET before PASV */
  CINIT!(FTP_USE_PRET, LONG, 188)

  /* RTSP request method (OPTIONS, SETUP, PLAY, etc...) */
  CINIT!(RTSP_REQUEST, LONG, 189)

  /* The RTSP session identifier */
  CINIT!(RTSP_SESSION_ID, OBJECTPOINT, 190)

  /* The RTSP stream URI */
  CINIT!(RTSP_STREAM_URI, OBJECTPOINT, 191)

  /* The Transport: header to use in RTSP requests */
  CINIT!(RTSP_TRANSPORT, OBJECTPOINT, 192)

  /* Manually initialize the client RTSP CSeq for this handle */
  CINIT!(RTSP_CLIENT_CSEQ, LONG, 193)

  /* Manually initialize the server RTSP CSeq for this handle */
  CINIT!(RTSP_SERVER_CSEQ, LONG, 194)

  /* The stream to pass to INTERLEAVEFUNCTION. */
  CINIT!(INTERLEAVEDATA, OBJECTPOINT, 195)

  /* Let the application define a custom write method for RTP data */
  CINIT!(INTERLEAVEFUNCTION, FUNCTIONPOINT, 196)

  /* Turn on wildcard matching */
  CINIT!(WILDCARDMATCH, LONG, 197)

  /* Directory matching callback called before downloading of an
     individual file (chunk) started */
  CINIT!(CHUNK_BGN_FUNCTION, FUNCTIONPOINT, 198)

  /* Directory matching callback called after the file (chunk)
     was downloaded, or skipped */
  CINIT!(CHUNK_END_FUNCTION, FUNCTIONPOINT, 199)

  /* Change match (fnmatch-like) callback for wildcard matching */
  CINIT!(FNMATCH_FUNCTION, FUNCTIONPOINT, 200)

  /* Let the application define custom chunk data pointer */
  CINIT!(CHUNK_DATA, OBJECTPOINT, 201)

  /* FNMATCH_FUNCTION user pointer */
  CINIT!(FNMATCH_DATA, OBJECTPOINT, 202)

  /* send linked-list of name:port:address sets */
  CINIT!(RESOLVE, OBJECTPOINT, 203)

  /* Set a username for authenticated TLS */
  CINIT!(TLSAUTH_USERNAME, OBJECTPOINT, 204)

  /* Set a password for authenticated TLS */
  CINIT!(TLSAUTH_PASSWORD, OBJECTPOINT, 205)

  /* Set authentication type for authenticated TLS */
  CINIT!(TLSAUTH_TYPE, OBJECTPOINT, 206)

  /* Set to 1 to enable the "TE:" header in HTTP requests to ask for
     compressed transfer-encoded responses. Set to 0 to disable the use of TE:
     in outgoing requests. The current default is 0, but it might change in a
     future libcurl release.

     libcurl will ask for the compressed methods it knows of, and if that
     isn't any, it will not ask for transfer-encoding at all even if this
     option is set to 1.

  */
  CINIT!(TRANSFER_ENCODING, LONG, 207)

  /* Callback function for closing socket (instead of close(2)). The callback
     should have type curl_closesocket_callback */
  CINIT!(CLOSESOCKETFUNCTION, FUNCTIONPOINT, 208)
  CINIT!(CLOSESOCKETDATA, OBJECTPOINT, 209)

  /* allow GSSAPI credential delegation */
  CINIT!(GSSAPI_DELEGATION, LONG, 210)

  /* Set the name servers to use for DNS resolution */
  CINIT!(DNS_SERVERS, OBJECTPOINT, 211)

  /* Time-out accept operations (currently for FTP only) after this amount
     of miliseconds. */
  CINIT!(ACCEPTTIMEOUT_MS, LONG, 212)

  /* Set TCP keepalive */
  CINIT!(TCP_KEEPALIVE, LONG, 213)

  /* non-universal keepalive knobs (Linux, AIX, HP-UX, more) */
  CINIT!(TCP_KEEPIDLE, LONG, 214)
  CINIT!(TCP_KEEPINTVL, LONG, 215)

  /* Enable/disable specific SSL features with a bitmask, see CURLSSLOPT_* */
  CINIT!(SSL_OPTIONS, LONG, 216)

  /* Set the SMTP auth originator */
  CINIT!(MAIL_AUTH, OBJECTPOINT, 217)

  /* Enable/disable SASL initial response */
  CINIT!(SASL_IR, LONG, 218)

  pub static CURLOPT_LASTENTRY: i32 = 0 as i32; /* the last unused */
