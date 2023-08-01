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
 
Ghetto software written in Rust that aims to assist in instance hardening, through the identification of security issues within integrated ServiceNow configurations.

It's important to **note** that this program is designed to assist you during a configuration review, and should **not** replace manual review. Therefore, it is recommended that this tool **not** be solely relied on, but rather used in support of your own analysis.

Further, it's **important** to note that the specific configurations and best practices may vary depending on the requirements and policies of an organisation.

## Download

```
git clone https://github.com/ret2eax/snowaudit.git
cd snowaudit
```

**SNOWAUDIT** is still currently in development and will be released officially soon. Functionality in pertinence to best security practices is still an ongoing effort. Feel free to star or watch this repo project for further updates. Pull requests are welcome if you wish to add contributions.

## Usage

There is no current support to fetch a `sys_properties` export directly from a ServiceNow instance. In the future, support will be added for this http parse feature. For the time being, the configuration file will need to be exported manually:


* Authenticate into your ServiceNow instance, or get the client to provide you with their ServiceNow `sys_properties` export.
* Export the ServiceNow `sys_properties` to `csv` format `{base_url}/sys_properties_list.do?CSV}`,
* Build and run `snowaudit`:

```
cargo build
cargo run sys_properties.list.csv
```

* View the `output.html` export.

### Example Output (Truncated)

| Definition | Current | Recommended | Description |
|------|-------|-------------------|-------------|
| glide.ui.rotate_sessions | false | true | Automatically rotates user sessions periodically |
| glide.ui.secure_cookies  | false | true | Ensures that all cookies used by the platform contain the secure flag |
| glide.script.use.sandbox | false | true | Enables the script sandbox feature to restrict execution of untrusted scripts |
| glide.login.no_blank_password | true | true | Prevents users from setting blank passwords |
| glide.security.csrf_enabled | false | true | Enables Cross-Site Request Forgery (CSRF) protection |
| glide.security.file.mime_type.validation | true | true | Validates MIME types |


## Terms of Use

By using snowaudit, you agree to the following terms and conditions:
- The software is provided on an "as-is" basis, without warranty of any kind, express or implied.
- The software is intended for informational purposes only and should not be considered as professional or legal advice.
- The software is not a substitute for proper security assessments, and it is the user's responsibility to ensure the security of their ServiceNow instance.
- The user is solely responsible for any consequences resulting from the use of this software.
- The software is subject to change without notice, and the developer does not guarantee that it will be updated or maintained.

## Liability Disclaimer

snowaudit is provided for informational purposes only, and the developer of snowaudit disclaims any liability for any damages or losses resulting from the use of snowaudit or the reliance on the information provided. The user acknowledges that they use snowaudit at their own risk, and the developer shall not be liable for any damages, including but not limited to direct, indirect, incidental, special, or consequential damages arising from its use. The user agrees to indemnify and hold the developer of snowaudit harmless from any claims, damages, or expenses arising from the use of snowaudit or the reliance on the information provided.
