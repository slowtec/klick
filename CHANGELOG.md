# Changelog

## v0.3.13 (2024-02-15)

- Changed
  - Lachgasszenarien UX rework #143
  - zur datenerfassung #141
  - more text #140
  - unterstützten -> unterstützt #127
  - CH4 Klärprozess -> CH4 Anlage (unspez.) #131
  - fixup #135 

## v0.3.12 (2024-02-14)

- Changed
  - Handlungsempfehlungen boxes - change order #139
  - Delete a doubled text there #135

## v0.3.11 (2024-02-13)

- Changed
  - Updated BarChart rendering 

## v0.3.10 (2024-02-13)

- Changed:
  - barchart diff view (instead of sankey diff tree) #32 
  - digester count default no 0 instead of 1: #132 
  - Various refactorings

## v0.3.9 (2024-02-05)

- Changed:
  - Landing page redesign #39
  - Better info text #106
  - BarChart fix #110
  - Adding CH4 EF #120

## v0.3.8 (2024-02-02)

- Changed:
  - #41 offene schlammtaschen

- Fixed:
  - #119 BHWK float fix

## v0.3.7 (2024-01-31)

- New:
  - BHKW - spatial arrangement of the EF field #88
  - Projektname (i) #106
  - n2o emissionsfaktor #109
  - typos fixing #108
  - FUEL_CONSUMPTION 0.033 -> 0.02 #116
  - always show ch4_slippage_sludge_storage #114
  - BHKW new EF #91

## v0.3.6 (2024-01-29)

- Changed:
  - Sankey font size 16px, see #111
  - BHKW input field label, see #88

## v0.3.5 (2024-01-29)

- Changed:
  - Sankey diagram renderer update, see #97

## v0.3.4 (2024-01-25)

- New:
  - Add leak test section
  - Add emission factor to Sankey header
  - Add section N2O emissions in the biologicaltreatment stage

- Changed:
  - Improve project list sorting
  - Format project dates with local time
  - Remove 'Offene Schlammstapelbehälter' from input fields
  - Hide section 'Minderungsmaßnahmen' during data collection
  - Minimize info boxes by default

- Fixed:
  - Fixed bug when saving the same project twice
  - Fix bug in navigation
  - Fixed some typos

## v0.3.3 (2024-01-24)

- New:
  - Delete outdated unconfirmed accounts

## v0.3.2 (2024-01-24)

- New:
  - Add functionality to manage projects online

## v0.3.1 (2024-01-18)

- New:
  - Add method to re-send email confirmation link

- Changed:
  - Deny login to accounts with unconfirmed e-mail addresses
  - #61 tiny fixes, typos, ...
  - #29 Added more `ch4_combined_heat_and_power_plant` tests
  - #29 simplified tests so errors point to the correct line number

## v0.3.0 (2024-01-17)

- New:
  - Added login functionality
  - Adapted flake.nix schema
  - Added SMTP functionality in backend
  - Added sqlite database support

- Changed:
  - #91 #92 new EF and new formula, BHKW cleanup
  - #90 sludge storage - correction in conversion factor

## v0.2.1 (2024-01-14)

- Changed:
  - Fixed links on landing page #73
  - Rename the "extrapoliert" scenario #82

## v0.2.0 (2024-01-11)

- New:
    - Added Jenkinsfile for jenkins support
    - Eigenstromerzeugung und Bilanzierung #61 fixes

- Changed:
   - Added 4 more test cases to 'just test' #29
   - Fixed "Handlungsempfehlungen"

## v0.1.9 (2023-12-14)

- Changed:
  - Fixed number format for Ablauf/Zulauf warning #16

## v0.1.8 (2023-12-13)

- New:
  - Added zulauf/ablauf wertevergleich #16
  - Added first recommendations for action #69

## v0.1.7 (2023-12-12)

- Changed:
  - Updated example #72

## v0.1.6 (2023-12-06)

- New:
  - Breadcumps tool navigation
  - Table of input values
  - Link version number in the footer with CHANGELOG.md

## v0.1.5 (2023-12-05)

- Changed:
  - Fixing "Werte zurücksetzen" functionality #68
  - Fixing input field spaces handling, by using trunc, before float parsing #70

## v0.1.4 (2023-11-30)

- Changed:
  - Fixing typos, #66

## v0.1.3 (2023-11-28)

- New:
  - Implemented form field validation with hints which still need to be filled out, see #13

## v0.1.2 (2023-11-27)

- New:
  - input form field validation with min/max value checks, see #13
  - page 'Open Source'
  - Add link to CHANGELOG.md
  - Thousands separator and decimal point (DE spelling/keyboard), see #60
  - Sankey: contains summary values N2O+CH4, see #49
- Changed: unit of 4 operating material values: kg -> t, see #23

## v0.1.1 (2023-11-22)

- New:
  - Clean node ordering in the Sankey chart
  - Simplified custom emission factor input
  - Meaningful colors in the Sankey chart
- Fixed: Several typos
- Changed: Emission factor label in Bar chart
- Removed: "Nutzung" from the Sankey chart

## v0.1.0 (2023-11-20)

Initial Release
