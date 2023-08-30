<!-- other logo colours and schemes
<p align="center">
  <img width="600" src="https://i.postimg.cc/qvY12TPD/snowaudit-v2.png">
</p>

<p align="center">
  <img width="600" src="https://i.postimg.cc/tJdChZhz/snowy-snowauditv2.png">
</p>

<p align="center">
  <img width="600" src="https://i.postimg.cc/L4ZWhzD7/snowy-snowauditv3.png">
</p>

<p align="center">
  <img width="600" src="https://i.postimg.cc/YqXLVyLL/snowy-snowauditv3-1.png">
</p>

<p align="center">
  <img width="600" src="https://i.postimg.cc/nLd0d1YH/snowy-pinksnowaudit.png">
</p>-->


<p align="center">
  <img width="600" src="https://i.postimg.cc/6qyYZ4DM/snowy-snowauditv4.png">
</p>


#
![](https://img.shields.io/github/languages/code-size/ret2eax/snowaudit?style=flat-square)
![](https://img.shields.io/github/stars/ret2eax/snowaudit?style=flat-square)
![](https://img.shields.io/github/watchers/ret2eax/snowaudit?style=flat-square)
![](https://img.shields.io/badge/release%20date-not%20yet%20released-blue?style=flat-square)
![](https://img.shields.io/github/downloads/ret2eax/snowaudit/total?style=flat-square)

# About
 
ServiceNow Configuration Analyser: Ghetto program written in Rust that aims to assist in ServiceNow instance hardening, through the identification of security issues within integrated ServiceNow configurations.

It's important to **note** that this program is designed to assist you during a configuration review, and should **not** replace manual review. Therefore, it is recommended that this tool **not** be solely relied on, but rather used in support of your own analysis. This is further reinforced on the basis that this program is not frequently maintained. 

Further, it's **important** to note that the specific configurations and best practices may vary depending on the requirements and policies of an organisation.

## Download

<!-- ### Release Builds (Recommended)

The following release builds can be pulled from [releases](https://github.com/ret2eax/snowaudit/releases):

* `x86_64 apple darwin` (macOS)
* `x86_64-pc-windows-gnu` (Windows)
* `x86_64-unknown-linux-gnu` (Linux)

### Manual Build -->

Optimised build:

```sh
git clone https://github.com/ret2eax/snowaudit.git
cd snowaudit
rustup target add #{target}
cargo build --release --target #{target}
cd target/{target}/release
./snowaudit
```

Or, simply build and run project:

```
git clone https://github.com/ret2eax/snowaudit.git
cd snowaudit/src
cargo run 
```

**snowaudit** is still currently in development. Functionality in pertinence to best security practices is still an ongoing effort. Feel free to star or watch this repo project for further updates. Pull requests are welcome if you wish to add contributions.

## Usage

There is no current support to fetch a `sys_properties` export directly from a ServiceNow instance. In the future, support will be added for this http parse feature. For the time being, the configuration file will need to be exported manually:

* Authenticate into your ServiceNow instance, or get the client to provide you with their ServiceNow `sys_properties` export.
* Export the ServiceNow `sys_properties` to `csv` format `{base_url}/sys_properties_list.do?CSV}`,
* Run `snowaudit` on the `sys_properties.list.csv` export:

```sh
./snowaudit sys_properties.list.csv
```

* View `snowaudit_report.html` output.


### Example Output (Truncated)

| DEFINITION | CURRENT | RECOMMENDED | DESCRIPTION |
|------|-------|-------------------|-------------|
| glide.ui.rotate_sessions | false | TRUE | Automatically rotates user sessions periodically |
| glide.ui.secure_cookies  | false | TRUE | Ensures that all cookies used by the platform contain the secure flag |
| glide.script.use.sandbox | false | TRUE | Enables the script sandbox feature to restrict execution of untrusted scripts |
| glide.login.no_blank_password | true | TRUE | Prevents users from setting blank passwords |
| glide.security.csrf_enabled | false | TRUE | Enables Cross-Site Request Forgery (CSRF) protection |
| glide.security.file.mime_type.validation | true | TRUE | Validates MIME types |
| glide.ui.session_timeout | 120	| Manually Review |	Override the default session timeout (30). This value is in minutes. |

**NOTE** `UNSUPPORTED` definitions are hidden by default, but can be toggled in the HTML export. Unsupported in this context means that the definition is not associated with ServiceNow instance hardenining or security related definitions.

## Terms of Use

By using snowaudit, you agree to the following terms and conditions:
- The software is provided on an "as-is" basis, without warranty of any kind, express or implied.
- The software is intended for informational purposes only and should not be considered as professional or legal advice.
- The software is not a substitute for proper security assessments, and it is the user's responsibility to ensure the security of their ServiceNow instance.
- The user is solely responsible for any consequences resulting from the use of this software.
- The software is subject to change without notice, and the developer does not guarantee that it will be updated or maintained.

## Disclaimer

**snowaudit** is provided for informational purposes only, and the developer of snowaudit disclaims any liability for any damages or losses resulting from the use of snowaudit or the reliance on the information provided. The user acknowledges that they use snowaudit at their own risk, and the developer shall not be liable for any damages, including but not limited to direct, indirect, incidental, special, or consequential damages arising from its use. The user agrees to indemnify and hold the developer of snowaudit harmless from any claims, damages, or expenses arising from the use of snowaudit or the reliance on the information provided.
