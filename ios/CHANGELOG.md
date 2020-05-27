# Changelog
All changes to the software that can be noticed from the users' perspective should have an entry in
this file. Except very minor things that will not affect functionality, such as log message changes
and minor GUI adjustments.

### Format

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).

Entries should have the imperative form, just like commit messages. Start each entry with words like
add, fix, increase, force etc.. Not added, fixed, increased, forced etc.

Line wrap the file at 100 chars.                                              That is over here -> |

### Categories each change fall into

* **Added**: for new features.
* **Changed**: for changes in existing functionality.
* **Deprecated**: for soon-to-be removed features.
* **Removed**: for now removed features.
* **Fixed**: for any bug fixes.
* **Security**: in case of vulnerabilities.


## [Unreleased]
### Added
- Add automatic key rotation every 4 days.

### Fixed
- Fix relay selection for country wide constraints by respecting the `include_in_country` 
  parameter.
- Fix defect when manually regenerating the private key from Settings would automatically connect
  the tunnel.
- Properly format date intervals close to 1 day or less than 1 minute. Enforce intervals between 1 
  and 90 days to always be displayed in days quantity.
- Fix a number of errors in DNS64 resolution and IPv6 support.
- Update the tunnel state when the app returns from suspended state.

## [2020.2] - 2020-04-16
### Fixed
- Fix "invalid account" error that was mistakenly reported as "network error" during log in.
- Fix parsing of pre-formatted account numbers when pasting from pasteboard on login screen.

### Added
- Format account number in groups of 4 digits separated by whitespace on login screen.
- Enable on-demand VPN with a single rule to always connect the tunnel when on Wi-Fi or cellular. 
  Automatically disable on-demand VPN when manually disconnecting the tunnel from GUI to prevent the 
  tunnel from coming back up.

## [2020.1] - 2020-04-08
Initial release. Supports...
* Establishing WireGuard tunnels
* Selecting and changing location and servers
* See account expiry
* Purchase more VPN time via in-app purchases
* See the current WireGuard key in use and how long it has been used
* Generate a new WireGuard key to replace the old
