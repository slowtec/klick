klick                         = KlicK
to-the-tool                   = Go to the tool
to-the-datacollection         = Go to data collection
to-the-sensitivity            = Go to sensitivity parameters
to-the-recommendations        = Go to recommendations
page-datacollection           = Data collection
page-sensitivity              = Sensitivity parameters
page-recommendations          = Recommendations
email-address                 = e-mail address
password                      = Password
communication-error           = A communication error has occurred
email-or-password-invalid     = Email address or password invalid
email-not-confirmed           = You have not yet confirmed your email address
something-went-wrong          = Sorry, something went wrong

log-in-to-your-account        = Please log in to your account
resend-email-for-confirmation = Resend email for confirmation
forgot-your-password          =  Forgot your password?
log-in                        = Log in

enter-login-information       = Please enter the desired login information
dont-have-an-account-yet      = Don't have an account yet?
already-have-an-account       = Already have an account?
user-account-offer-question   = What does a user account offer you?
user-account-benefits         = With an account you can manage your data online.
successfully-signed-up        = Successfully registered
sign-up-success-message       = Congratulations! You have successfully registered your account. Now check your e-mail inbox and confirm the validity of your e-mail address.

reset-password-promt          = Please enter your e-mail address to reset your password
reset-password                = Reset password
how-does-it-work              = How does it work?
reset-password-description    = You will receive an e-mail with a link that you can use to set your new password.
email-sent-to-reset-password  = E-mail sent to reset the password.
reset-request-success-message = Now check your e-mail inbox and open the corresponding e-mail. Then click on the link it contains to enter your new password.
form_data_table_overview      = Overview of entered values (Data collection and sensitivity parameters)
co2-savings                   = CO₂ Savings by

# aria_label

aria_label_barchart           = A bar chart within the sensitivity, which is only displayed if an improvement / deterioration has occurred due to a selection.

########################################### profile ###########################################

datacollection_enforcement_helper = Please add the missing values ​​in the input form so that the emissions can be calculated and visualized.
datacollection_missing_fields = Please add the following values ​​so that the total emissions of your wastewater treatment plant can be calculated based on different scenarios:
sludge-bags-are-closed        = Slude bags are closed
sludge-bags-are-closed-info   = If the sludge bags of the digester(s) of your sewage treatment plant are closed and not open to the ambient air, please check this box.
sludge-storage-is-closed      = Sludge storage is closed
sludge-storage-is-closed-info = If the sludge storage tanks of your sewage treatment plant are tightly covered, please click this box.

########################################### sensitivity ###########################################

sensitivity-barchart-title    = Changes due to options in sensitivity parameters
sensitivity-barchart-description = The following graph shows the changes in greenhouse gas emissions [t CO₂ equivalents/year] or % of total emissions due to the selected action measures.

sensitivity-of-emission-factors = Sensitivity of emission factors
sensitivity-of-emission-factors-info = In the following "expandable" sections, you have the option of defining various emission factors (EF) in more detail. You can calculate how the respective adjustment of the EF of system components or the entire wastewater treatment plant affects the carbon footprint. You can also skip the sensitization/refinement and go directly to the recommendations for action (in this case, the KlicK tool calculates based on the standard factors/parameters mentioned).

sensitivity-custom-emissions = Additional custom emissions
sensitivity-custom-emissions-description = Extension of emissions using your own types/values.

sensitivity-ch4-chp              = Methane emissions from CHP
sensitivity-sludge-storage       = Methane emissions from sludge storage
sensitivity-fossil-co2           = Fossil CO₂-Emissionen from sewage

# fossil_co2_emissions.rs
sensitivity-fossil-co2-1         = The majority of the CO₂ released from wastewater is of biogenic origin. This CO₂ is therefore considered climate-neutral and is not included in the greenhouse gas balance. A smaller part of the CO₂ is due to fossil and synthetic compounds. Estimates assume that this can account for up to 5–10% of the organic load in municipal wastewater (Law et al. 2013). For wastewater with high proportions of commercial/industrial dischargers (> 45%), the fossil CO₂ load can be even higher (UBA 2022).
sensitivity-fossil-co2-2         = Below you can estimate how high/low the proportion of fossil CO₂ is based on the total organic carbon (TOC<sub>Influent</sub>) of the sewage treatment plant Influent. The fossil CO₂ is emitted from the biological treatment, from the sewage gas and sewage sludge utilization. Using a selectable CO₂-EF (fossil), you can calculate how this affects the carbon footprint of your sewage treatment plant.
sensitivity-fossil-co2-3         = If you have not entered a value in the TOC<sub>Influent</sub> input field during data collection, the tool estimates the fossil CO₂ emissions from biological treatment via the specified COD<sub>Influent</sub> using the theoretical conversion factor of COD:TOC of 1:2.6 derived from the chemical equation C + O₂ → CO₂. If you leave the "CO₂-EF (fossil)" input field below blank, an average CO₂-EF (fossil) of 3.85 (according to Law et al. 2013) is assumed. 
sensitivity-fossil-co2-infobox-text = In addition to the fossil CO₂ emissions from biological treatment, a significant proportion of these emissions were measured in sewage sludge and sewage gas
sensitivity-fossil-co2-infobox-cite-source = Extract from UBA Text 149/2022 (p. 5)
sensitivity-fossil-co2-infobox-cite-text = The test results show that sewage sludge from municipal plants with minor commercial wastewater (< 45%, calculated as the average capacity of the population minus the number of inhabitants connected) contains around 80% biogenic carbon and digester gases contain around 85% biogenic carbon. The fossil carbon is probably due to synthetic products or fossil raw materials that are difficult to degrade. […] Shares of around 28 to 71% in sewage sludge and around 11 to 88% in digester gas were determined.
sensitivity-fossil-co2-4         = These fossil CO₂ components could, for example, be separated from the sewage gas and put to technical use to further reduce the GHG emission potential of the sewage treatment plant.

# ch4_emissions_chp.rs
sensitivity-ch4-chp-aria = A bar chart that graphically shows different scenarios for calculating methane emissions and is also used to select one of these scenarios.
sensitivity-ch4-chp-infobox-1-text = CHP plants have different methane slips depending on the model and age
sensitivity-ch4-chp-infobox-1-cite-source = Extract from DWA leaflet 230-1 (2022, p. 25)
sensitivity-ch4-chp-infobox-1-cite = Process-related methane emissions also arise during gas utilization: CHP engines operate according to the four-stroke principle. In this case, both the inlet and outlet valves are (partially) opened for a short time during the transition from the fourth (exhaust) to the first (intake) stroke to allow gas exchange. This overlap can potentially cause small amounts of unburned digester gas to enter the exhaust gas stream. Gasoline engines have a methane slip in the range of 1% to 2%. Pilot jet engines (not relevant for digester gas) are higher, in the order of 2% to 3%. Micro gas turbines (typical power class from 30 kW to 65 kW), on the other hand, can achieve a methane slip of < 1% (STMWI 2016).

sensitivity-ch4-chp-p-1 = By selecting or entering your own emission factor (EF) for the CHP plant of your wastewater treatment plant, your carbon footprint with regard to methane emissions can be estimated in more detail:
sensitivity-ch4-chp-scenario = The scenario
sensitivity-ch4-chp-scenario-2 = is selected [in t CO₂ equivalents/year]. A different scenario can be selected by clicking.

sensitivity-ch4-chp-infobox-2-text = Additional information on methane slip:
sensitivity-ch4-chp-infobox-2-cite-source = Extract from DWA leaflet 230-1 (2022, p. 25)
sensitivity-ch4-chp-infobox-2-cite = The gas composition, combustion chamber temperature (gas humidity), combustion chamber design and operating mode influence the combustion processes. At high oxygen concentrations (lean operation), which are necessary to reduce NOₓ formation at high temperatures, the methane slip increases. In addition to the operating mode, the unit output also has an influence on the methane slip. Measurements during operation have shown that less methane is generally emitted via the exhaust gas under full load than under partial load. In micro gas turbines, this effect is very pronounced and can lead to an increase of up to > 5% in 60% partial load operation (STMWI 2016).

# ch4_emissions_open_digesters.rs

sensitivity-open-digesters     = Methane emissions from open digestion towers and sludge storage
sensitivity-open-digesters-p-1 = Methane can escape through <b>open sludge pockets in digesters</b>. An emission factor CH₄-EF [in m³/h] can then be calculated for the methane slip (e.g. from a measurement campaign or as an estimated value).
sensitivity-open-digesters-p-2 = If you leave the field blank, an average EF of 1.25 m³/h according to Li (2020) is used. In both cases, the number of digestion towers is taken into account proportionally (see input "number of digestion towers" in data collection).
sensitivity-open-digesters-p-3 = If you have selected 'open sludge storage' (in data collection), you can estimate the impact of methane slip on the climate balance of your wastewater treatment plant. The following input field allows you to calculate a CH₄-EF [%] for your sludge storage tank (e.g. based on a residual gas potential analysis). If you leave the field blank, the reference value from Parravicini et al. (2016) CH₄-EF = 2% of the total sewage gas volume will be used.

sensitivity-open-digesters_1-text = Sludge storage contributes significantly to methane emissions
sensitivity-open-digesters_1-cite-source = Extract from DWA leaflet 230-1 (p. 24)
sensitivity-open-digesters_1-cite-text = Depending on the technical degree of digestion of the sludge digestion and the storage time, up to 15 kg CO<sub>2</sub> equivalents/(E·a) can still be emitted during digested sludge storage (source: DWA 2020). This corresponds to a methane formation potential of 576 g CH<sub>4</sub>/(E·a). For the methane emissions from the storage and dewatering of digested sludge, PARRAVICINI et al. (2016) specify a range of 2% to 4.5% of methane production.

# ch4_emissions_open_sludge_storage.rs

ch4_emissions_open_sludge_storage_1-text = Emissions from the storage of aerobically stabilized sludge have a significant emission potential
ch4_emissions_open_sludge_storage_1-cite-source = Extract from DWA leaflet 230-1 (2022, p. 24-25)
ch4_emissions_open_sludge_storage_1-cite-text = Even when operated properly, aerobically stabilized sludge contains approximately 11 g oDM/(E·d) more easily degradable substances than digested sludge (approx. 4 g oDM/(E·d) in digested sludge), unless the aerobic sludge age is well over 30 d (DWA 2020). If the sludge is stored or saved for a longer period of time, an anaerobic environment can develop, which promotes methane formation. When aerobically stabilized sludge is stored or saved, methane can be produced and emitted. The emission potential is therefore significantly higher than the methane emissions expected from the operation of a properly operated digestion plant. The storage of insufficiently stabilized sludge can result in higher methane emissions due to the higher proportion of organic matter. To reduce these emissions, the formation of an environment necessary for methane formation must be avoided.

# n2o_emissions.rs

sensitivity-n2o = Nitrous oxide emissions
n2o_emissions-h3-1 = Nitrous oxide emissions during the biological treatment stage
n2o_emissions-p-1 = Nitrous oxide emissions contribute significantly to the overall greenhouse gas potential of sewage treatment plants. The first estimate of this potential during data collection is made using an emission factor for nitrous oxide (N₂O-EF) according to Parravicini et al. (2016, TU Vienna), see the first bar in the diagram below.
n2o_emissions-p-2 = Since the occurrence of N₂O emissions is usually plant-specific <b> [N₂O plant] </b>, the KlicK tool offers further evaluation scenarios for nitrous oxide emissions. These are shown in the following bar chart, including the resulting nitrous oxide emissions [as CO₂ equivalents].
n2o_emissions-p-3 = By clicking on the individual bars in the diagram, the respective scenario is used for the overall balance below (in the Sankey diagram).
n2o_emissions-p-4-1 = The scenario
n2o_emissions-p-4-2 = is selected [in t CO₂ equivalents/year]. A different scenario can be selected by clicking.

n2o_emissions-h3-2 = Nitrous oxide emissions during process water treatment
n2o_emissions-p-5 = In addition, you can enter and balance a user-defined value for the N₂O EF (e.g. based on your own estimate or a measurement campaign). The EF factor appears in the bar chart and can then also be selected.
n2o_emissions-p-6 = Process water treatment in wastewater treatment plants can be associated with significant additional nitrous oxide emissions. Vasilaki et al. (2019) indicate in their meta-study a nitrous oxide EF of 1.7-5.1% of the total nitrogen in process water.
n2o_emissions-p-7 = By entering the annual amount of nitrogen treated in the process water [t/a], you can estimate the resulting share of greenhouse gas emissions [CO₂ eq./a].
n2o_emissions-p-8 = You can freely select the N₂O-EF used for this purpose using the input field “N₂O-EF process water” below or leave it blank to calculate with an average EF of 2% (according to Vasilaki et al. 2019).

########################################### recommendation ###########################################

recommendation-barchart-title    = Changes due to options in recommendations
recommendation-barchart-description = The following graph shows the changes in greenhouse gas emissions [t CO₂ equivalents/year] or % of total emissions due to the selected action measures.

# ch4_emissions_open_digesters.rs
recommendation-methan-emissions  = Methane emissions from open digesters and during sludge storage
recommendation-ch4-open-digesters-p-1 = Closing sludge pockets in digestion towers and sludge storage has a positive effect on the carbon footprint of sewage treatment plants by reducing methane slip. You can do this using the checkboxes below.
recommendation-ch4-open-closing-sludge-bags = Close sludge bags
recommendation-ch4-open-closing-sludge-storage = Close sludge storage

# excess_energy_co2_equivalent.rs
recommenation-excess-energy      = Excess Energy Emissions
recommenation-excess-energy-p-1 = <b>Energy saving measures</b> and <b>Renewable energies</b> can make a significant contribution to reducing indirect emissions and achieving energy self-sufficiency. In order to demonstrate the positive effects of increasing renewable energies: photovoltaics (PV), wind, hydropower and/or waste heat utilization, various scenarios can be assessed below. If you do not want to assess the respective technology, you can leave the respective field blank.
recommenation-excess-energy-p-2-1 = Your sewage treatment plant is energy neutral. The sewage treatment plant saves
recommenation-excess-energy-p-2-2 = t CO₂-eq./a ein.
recommenation-excess-energy-p-3-1 = Your sewage treatment plant still requires external power (supplier), for which
recommenation-excess-energy-p-3-2 = t CO₂-eq./a of energy-related emissions are generated.

# leak_test.rs
recommendation-leak-test         = Leak test
recommendation-leak-test_1-text = The (annual) inspection of possible leaks and their elimination can contribute significantly to the GHG reduction potential at wastewater treatment plants.
recommendation-leak-test_1-cite-source = Extract from DWA leaflet 230-1 (p. 23 and 43)
recommendation-leak-test_1-cite-text = Methane, which can escape from various containers and pipes due to leaks and/or slip losses. The robots explore the area and identify leaks even in places where monitoring was not previously possible due to the location, and visualize the results accordingly.
recommendation-leak-test_2-text = Potential leaks can occur in sewage treatment plant components such as manholes.
recommendation-leak-test_2-cite-source = Extract from DWA leaflet 230-1 (p. 23 and 43)
recommendation-leak-test_2-cite-text = Further emissions from the digestion process can arise from the discharge of floating sludge and from leaks in the gas system operated at slight overpressure.
recommendation-leak-test-p-1 = To determine exactly where and how much methane is emitted, measurements are recommended which can be used to precisely determine the GHG reduction potential.

# ch4_emissions_pre_treatment.rs
recommendation-ch4-pre-treatment = Methane emissions from pre-treatment
recommendation-ch4-pre-treatment-1 = Good management of your sewage treatment plant is a key factor in improving your carbon footprint. With regard to the mechanical treatment stage and pre-treatment, this can manifest itself in the following way for methane emissions:
recommenations_ch4_emissions_pre_treatment_infobox_1-text = Avoidance of long sludge residence times to reduce the methane formation potential
recommenations_ch4_emissions_pre_treatment_infobox_1-cite-source = Extract from DWA leaflet 230-1 (p.28)
recommenations_ch4_emissions_pre_treatment_infobox_1-cite-text = In primary clarifiers, ISGREN & MARTENSSEN (2013) determined an average CH₄ concentration (from just three samples) of 0.83 mg CH₄/L in the primary clarifier of the Sjölunda wastewater treatment plant. The CH₄ concentration was therefore higher than in the inlet area of ​​the wastewater treatment plant, so that the authors suspect methane formation in the primary clarifier. However, the small number of samples should be noted here. Due to the short sludge retention times, the formation of the biocenosis required for methane formation is rather unlikely.
recommenations_ch4_emissions_pre_treatment_infobox_2-text = Regulate continuous primary sludge removal so that sufficient carbon is available for denitrification on the one hand and for digestion (if applicable) on the other
recommenations_ch4_emissions_pre_treatment_infobox_2-cite-source = Extract from DWA leaflet 230-1 (p.18)
recommenations_ch4_emissions_pre_treatment_infobox_2-cite-text = A comparable study from Denmark showed that even large sewage treatment plants with high nitrogen purification performance can have increased emission factors of more than 0.5%. One possible reason identified here was the removal of sludge from the primary treatment to increase the biogas yield in the digestion. The reduced COD/N ratio resulting from the removal of carbon is then not sufficient for complete denitrification.

# n2o_emissions_in_the_biological_treatment_stage.rs
recommendation-n2o-biological    = Nitrous oxide emissions during the biological treatment stage
recommendation-n2o-biological_p_1 = Nitrous oxide emissions contribute significantly to the overall greenhouse gas potential of wastewater treatment plants. The occurrence of N₂O emissions is plant-specific, so that at the current state of research and monitoring, the following measures can be summarized with a focus on operational settings:
recommendation-n2o-biological_1-cite-source = Extract from DWA leaflet 230-1 (2022, p. 23/24)
recommendation-n2o-biological_p_2 = Ensuring sufficient sludge age for nitrification
recommendation-n2o-biological_p_3 = Equalisation of the influent load when discharging highly concentrated partial flows such as industrial discharges, sludge water from drainage or easily degradable C sources to support denitrification
recommendation-n2o-biological_p_4 = Avoidance of nitrite concentrations
recommendation-n2o-biological_p_5 = sufficient denitrification volume
recommendation-n2o-biological_p_6 = Clear ventilation regime with clear aerobic and anoxic zones/times and variability of ventilation to provide volumes adapted to the load
recommendation-n2o-biological_p_7 = Previous studies of nitrous oxide emissions from sewage treatment plants have shown that these are often subject to seasonal fluctuations and are plant-specific. From this point of view, measurement and monitoring campaigns are useful, which create a database (ideally over a year) to determine the plant-specific emission factor. This can be used for municipal climate reporting. In addition, potential reduction measures can be better assessed.

# n2o_emissions_side_stream_system.rs
recommendation-n2o-side-stream   = Nitrous oxide emissions during process water treatment
recommendation-n2o-side-stream_p_1 = Since process water treatment plants are relatively small tanks, nitrous oxide emissions can be eliminated by covering and exhaust air treatment (oxidation).
recommendation-n2o-side-stream_p_2 = In the interests of sustainability and circularity, a nitrogen recovery plant can be integrated.

########################################### project menu ###########################################

project-label = Project
project-reset-values = Reset values
project-load-example-values = Load example
project-load-from-file = File import
project-save-to-file = File export
project-export-csv = Export to CSV-file
project-load-from-online = Load from cloud
project-save-to-online = Save to cloud
back-to-table = back to overview