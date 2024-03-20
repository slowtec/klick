# Changelog

## v0.4.2 (2024-03-20)

- Changed:
  - Show default values in form data over (#241)
  - Info feld "Gasbezug" kwh -> m3 (#248)
  - Added default CHP emission factor calculation method to example data
  - Improve menu entries (#243)
- Fixed:
  - Ignore but warn on loading corrupt project data
  - Typo in code related to `EMISSION_FACTOR_SLUDGE_STORAGE` (#214)

## v0.4.1 (2024-03-18)

- Changed:
  - Updated example #72: Clicking 'Datenerfassung,Sensitivitaet,Handlungsempfehlungen' now scrolls to the same anchor.
  - Show Cards always: Improved consistency in displaying cards.
  - Always trigger value change (None) if the field is empty: Ensures consistent behavior for empty input fields.
  - Fix input field handling: Addressed issues related to input field handling.
  - Schlammtaschen/Schlammlagerung Datenerfassung vs. Handlungsempfehlungen: Resolved discrepancies between data collection and recommendations. #236
  - Klaerschlamm -> Klärschlamm: Corrected spelling for clarity. #208
  - Publish klick-domain v0.4.0: Updated dependencies to the latest version.

## v0.4.0 (2024-03-18)

- Added
  - Sensitivity information to overview table.
  - Mention of CH4 plants in bar chart.
  - Text formatting in "Nitrous Oxide Emissions" box updated.
  - Extended text for "fossil CO2".
  - Development support for Mac OS X added.

- Changed
  - Major refactoring conducted.
  - Side stream treatment changed to Process Water Treatment.
  - Clarified naming for "Energy-related Emissions".
  - Various texts revised and clarified.
  - "Operating materials" no longer marked as a mandatory field. (#212 Reverted)
  - Removed house icon from breadcrumbs.
  - Sludge transport distance limit updated from 500 to 2000.

- Fixed
  - Loading/saving of projects corrected.
  - Handling of not always valid form data in the database.
  - Fixed two-way bindings of form data.
  - Scroll positions of page sections corrected.
  - Various minor CSS fixes.

- Removed
  - PDF & CSV export menu entries temporarily removed

## v0.3.18 (2024-02-29)

- Changed
  - EMISSION_FACTOR_SLUDGE_STORAGE 1.6 -> 2.0 #144
  - sensitivity updates: lachgasemissionen & methanemissionen bhkw #175

## v0.3.17 (2024-02-22)

- Changed
  - barchart percentage value support #164
  - Add PDF export to projects menu
  - export feature to CSV #84
  - fix for floating points in sankey diagrams #142

## v0.3.16 (2024-02-20)

- Changed
  - rework Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung #151
  - t -> t CO₂-Äq./a #155
  - Schlammstapelbehälter -> Schlammlagerung #148

## v0.3.15 (2024-02-19)

- Changed
  - Pflichtfeld Klärgas/Methangehalt/Eigenstromerzeugung/Transportdistanz anpassung #145

## v0.3.14 (2024-02-18)

- Changed
  - calculatioin of CH4 slippage sludge bags emissions
  - `EMISSION_FACTOR_SLUDGE_STORAGE` 1.7 -> 1.6 #144
  - Set digester count to 1 in example data

- Fixed
  - Some texts and typos #124
  - Some refactorings

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
