---
name: dc-fast-charger-power-supply-and-grounding.md
description: Use when the agent is installing DC fast chargers, planning medium-voltage or 480V three-phase service for high-current charging, specifying equipment grounding for high available fault current, coordinating isolation monitoring and ground-fault detection, or working with the utility on DCFC service design and transformer sizing.
---

# DC Fast Charger Power Supply and Grounding

A DC fast charger (DCFC) is not a receptacle with a cord; it is a high-power rectifier that converts three-phase utility AC into hundreds of amps of isolated DC delivered directly to a vehicle battery. A 150 kW charger pulls roughly 180 amps from a 480V three-phase service, a 350 kW charger pulls over 400 amps, and a site with several of these is a small industrial substation. The judgment problem is that the power supply, the grounding, and the utility coordination for DCFC are an order of magnitude more demanding than Level 2 AC charging, and an electrician who scales up Level 2 thinking will undersize the service, misjudge the available fault current, and create a grounding scheme that cannot clear a fault or that interferes with the charger's isolation monitoring. DCFC also introduces medium voltage in many sites, with its own clearances, PPE, and qualification requirements. This skill covers the supply design, the grounding and bonding for high-current high-fault environments, the isolation monitoring that protects the user, and the utility coordination that determines whether the site can be built at all.

## Core Rules

### Size the AC Service for the Charger Input Plus Efficiency Losses and Diversity

A DCFC delivers rated DC power to the vehicle, but the AC input is higher because the charger's rectifier, power factor correction, and cooling all consume energy, and the input power factor is rarely unity. A 150 kW charger may draw 175 kVA from the line, and a 350 kW charger may draw 400 kVA. The service must be sized to the input kVA, not the output kW, with the continuous-load factor applied, and with realistic diversity only if a load management system enforces it. The trap is reading the charger's 150 kW rating and sizing the service to 150 kW, leaving no margin for efficiency losses, power factor, or future expansion. The defense is to obtain the charger's input kVA and current from the data sheet, size the breaker and conductors to 125 percent of that input as a continuous load, and apply the diversity factor only where a controller caps the aggregate demand.

### Coordinate the Utility Service and Transformer Early, Before Design Freeze

A DCFC site often requires a new or upgraded utility transformer, a new medium-voltage lateral, and possibly a padmount or switchgear upgrade, and these are long-lead items controlled by the utility, not the contractor. A 350 kW site may need a 500 kVA or larger transformer, and the utility may require months to engineer and schedule the upgrade, with the site design dependent on the transformer impedance and the available fault current the utility provides. The trap is finalizing the site electrical design before the utility confirms the service characteristics, then discovering the available fault current is higher than the charger's breaker rating, or the transformer is smaller than assumed. The defense is to initiate the utility coordination at project conception, obtain the written service commitment including transformer size and impedance and fault current, and design the site equipment around those confirmed values.

### Specify Equipment Grounding for the High Available Fault Current

DCFC sites are fed from dedicated transformers that sit close to the load, which means low source impedance and high available fault current, often 30,000 to 65,000 amps or more at the charger disconnect. Every overcurrent device, transfer means, and equipment grounding conductor must be rated for that available fault current, and the grounding path must be robust enough to let the upstream device clear a bolted fault without the conductor fusing. The trap is installing standard breakers and standard grounding conductors rated for 10 kAIC in a 65 kA environment, where a fault exceeds the breaker's interrupting capacity and the device ruptures instead of clearing. The defense is to obtain the available fault current from the utility or a fault study, specify breakers with adequate interrupting ratings (often current-limiting fuses or high-AIC breakers), and size the equipment grounding conductor per 250.122 with attention to the high fault magnitude.

### Design the Grounding Electrode System for Lightning and Surge Environment

DCFC cabinets are large metallic structures outdoors, often the tallest object in a parking area, fed by long underground runs that are susceptible to induced surges from nearby lightning. The grounding electrode system at the charger must bond the cabinet to earth through a low-impedance path, typically a concrete-encased electrode, ground ring, or driven rods, and surge protective devices must be installed at the service and at the charger input. The trap is relying on a single driven rod with a marginal resistance and no surge protection, so a nearby lightning strike induces a surge that destroys the charger's input rectifier. The defense is to install a robust grounding electrode system per 250 Part III, bond all electrodes together, measure the resistance to earth, and install listed surge protection sized for the location.

### Preserve the Galvanic Isolation and Isolation Monitoring Function

A DCFC is required to be galvanically isolated from the utility AC, meaning the DC output to the vehicle is transformer-isolated from the input, and the charger continuously monitors the isolation resistance between the DC output and earth. If the isolation breaks down, the charger shuts down before the user can receive a shock. This isolation monitoring depends on the charger not being bonded to the building grounding system in a way that defeats the measurement. The trap is bonding the charger's DC negative to ground, or running a separate equipment grounding conductor that creates an unintended reference, defeating the isolation monitor and creating a shock path. The defense is to follow the charger manufacturer's grounding instructions exactly, preserve the isolation barrier, and verify the isolation monitoring is active and trips the charger on a simulated fault during commissioning.

### Manage the Heat Rejection and Enclosure Environment

A DCFC is roughly 90 to 95 percent efficient, which means a 150 kW charger rejects 7 to 15 kW of heat into its cabinet and into the enclosure, and a 350 kW charger rejects proportionally more. The cabinet has internal cooling, but the electrical room or pad area must dissipate the heat, and high ambient temperatures derate the charger's output. The trap is mounting the charger in a small unventilated enclosure or in direct sun, so the internal temperature rises, the charger derates to protect itself, and the advertised 150 kW becomes 90 kW on a summer afternoon. The defense is to plan the thermal environment, provide ventilation or air conditioning for indoor equipment, shade outdoor cabinets, and confirm the charger's derating curve against the expected ambient.

### Address Medium-Voltage Safety and Qualification on Larger Sites

Sites with multiple high-power chargers, or with chargers above roughly 350 kW, often receive power at medium voltage (typically 12.47 kV or 13.8 kV) that is stepped down at the site. Medium-voltage work has its own code requirements (NEC Article 300 Part IV, Article 328, Article 490), its own clearances, its own PPE and approach boundaries, and its own qualification expectations. The trap is treating a 13.8 kV primary as if it were another 480V feeder, with inadequate clearance, no switching procedure, and unqualified personnel. The defense is to identify medium-voltage work early, engage qualified personnel, follow the utility's switching and lockout requirements, and apply the appropriate approach distances and PPE.

## Common Traps

### Sizing the Service to the DC Output Instead of the AC Input

The installer reads the charger's 150 kW output rating and sizes the breaker to 150 kW, ignoring the input kVA. The mechanism of the failure is that the charger draws more AC current than its DC output suggests, due to efficiency losses and a non-unity power factor, so the undersized breaker carries more current than expected and trips under sustained fast charging. The false signal is that the breaker holds during a brief test, which proves only the instantaneous load fits, not the continuous input. The harm is nuisance tripping, derated charging, and a service that cannot deliver the rated power the owner paid for.

### Underrating the Breaker Interrupting Capacity for the Available Fault Current

The site is fed from a dedicated 1000 kVA transformer with low impedance, and the available fault current at the charger disconnect exceeds 50 kA, but standard 10 or 14 kAIC breakers are installed. The mechanism of the failure is that a bolted fault at the charger draws current far exceeding the breaker's interrupting rating, the breaker cannot extinguish the arc, and the device ruptures violently instead of clearing. The false signal is that the breaker is rated for the continuous load current, which is true but irrelevant to fault clearing. The harm is equipment destruction, arc-flash energy, and a fault that does not clear until an upstream device operates, if it ever does.

### Defeating the Isolation Monitoring With an Unintended Ground Bond

The installer bonds the charger's DC negative to the cabinet ground to "improve grounding," or runs a parallel equipment grounding path that references the DC output to earth. The mechanism of the failure is that the charger's isolation monitor measures resistance between the DC output and earth, and the unintended bond drives that resistance toward zero, either masking a real isolation failure or tripping the monitor falsely. The false signal is that bonding everything to ground is safer, which is true for AC equipment grounding but destructive for the isolated DC output. The harm is a charger that either refuses to run or, worse, runs with defeated protection that could expose the user to shock.

### Skipping the Utility Coordination and Discovering a Transformer Constraint Late

The site is designed for four 150 kW chargers without confirming the utility can supply the load, and the utility later reports the existing transformer cannot support the site and a new service will take nine months. The mechanism of the failure is that the utility service is the long-lead, utility-controlled constraint, and design freeze without utility commitment leaves the schedule hostage to a transformer upgrade. The false signal is that the site has a utility connection point, which proves physical presence but not capacity. The harm is a delayed or cancelled project, change orders, and a charger site that sits idle waiting for power.

### Mounting the Charger in an Unventilated or Sun-Exposed Location

The charger is installed in a small unventilated electrical room or in full sun with no shading, and on hot days it derates sharply. The mechanism of the failure is that the charger's internal electronics have a temperature limit, and as the ambient rises the charger reduces output to protect itself, so the advertised 150 kW drops to 90 or 100 kW. The false signal is that the charger runs at full power on a cool morning, which proves the unit is functional but not that it can sustain output in the real thermal environment. The harm is a site that underperforms its rated capacity exactly when demand is highest, damaging the owner's return on investment.

### Treating Medium-Voltage Primary as Ordinary 480V

A multi-charger site has a 13.8 kV primary feeding a padmount transformer, and the crew treats the primary conductors and terminations like 480V feeders. The mechanism of the failure is that medium voltage requires greater clearance, different cable terminations (often separable elbows), specific approach boundaries, and qualified switching, and treating it as low voltage leads to inadequate clearance, unsafe work, and possible flashover. The false signal is that the conductors look similar and terminate in a box, which proves physical similarity but not equivalent safety requirements. The harm is a serious arc-flash incident or a code violation that fails inspection and endangers future workers.

## Self-Check

- Did I size the AC service to the charger's input kVA at 125 percent as a continuous load, accounting for efficiency losses and power factor, rather than to the DC output kW rating?
- Did I initiate utility coordination at project conception and obtain a written service commitment including transformer size, impedance, and available fault current before design freeze?
- Did I obtain the available fault current at the charger disconnect and specify overcurrent devices with adequate interrupting ratings, including current-limiting fuses or high-AIC breakers where needed?
- Did I design a robust grounding electrode system per NEC 250 Part III, bond all electrodes together, measure the earth resistance, and install listed surge protection at the service and charger input?
- Did I follow the charger manufacturer's grounding instructions exactly to preserve the galvanic isolation and the isolation monitoring function, and did I verify the monitor trips on a simulated fault during commissioning?
- Did I plan the thermal environment for the charger's heat rejection, provide ventilation or shading, and confirm the derating curve against the expected ambient?
- For any medium-voltage work, did I identify it early, engage qualified personnel, follow utility switching and lockout requirements, and apply the correct approach distances and PPE?
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
