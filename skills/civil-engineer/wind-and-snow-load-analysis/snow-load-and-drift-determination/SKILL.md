---
name: snow-load-and-drift-determination.md
description: Use when the agent is determining roof snow loads from ASCE 7 Chapter 7, computing ground snow and flat roof loads, applying exposure thermal and importance factors, evaluating balanced and unbalanced loading, or sizing leeward windward and sliding snow drifts on lower roofs and parapets.
---

# Snow Load and Drift Determination

Snow load determination is the process of converting a regional ground snow load into the design roof load that a structure must carry, accounting for the way wind, roof geometry, building heat loss, and adjacent roofs redistribute and concentrate snow. The governing reference is ASCE 7 Chapter 7, which expresses the balanced roof snow load as the ground snow pg modified by an exposure factor, a thermal factor, an importance factor, and a roof slope factor, and then layers drift and sliding surcharges on top of the balanced load where geometry creates accumulation. The harm this skill prevents is a roof designed for a uniform balanced load that collapses under a drift that triples the load over a local area, or a roof whose thermal condition or exposure was misjudged so the balanced load itself was understated. Because snow loads are dominated by drifts and sliding rather than the uniform field, the engineer who stops at the balanced load has not finished the snow analysis, and the most common roof failures in snow country are local overloads at valleys, parapets, and roof steps that the balanced load never represented.

## Core Rules

### Establish the Ground Snow Load and the Flat Roof Load

Begin with the ground snow load pg from the ASCE 7 ground snow map or, where available, a site-specific value from the authority having jurisdiction, because many jurisdictions have state or local snow load studies that supersede the national map with higher, topographically resolved values. Confirm that pg is the 50-year mean recurrence interval value and that the correct value is used for the project's location, because snow load varies sharply with elevation and orographic effect and a value taken from a nearby valley can understate a hillside site by a large margin. Compute the flat roof snow load pf as 0.7 × Ce × Ct × Is × pg, where Ce is the exposure factor, Ct the thermal factor, Is the importance factor based on Risk Category, and the 0.7 accounts for the typical reduction from ground to roof. Do not apply pg directly as a roof load, because the ground-to-roof reduction and the modifying factors exist precisely because a roof does not accumulate snow the way the ground does.

### Apply the Exposure, Thermal, and Importance Factors With Judgment

The exposure factor Ce accounts for terrain and wind: a fully exposed roof above the surrounding terrain in an open area (Ce = 1.2) sheds snow by wind, while a sheltered roof in a dense urban or wooded setting (Ce = 1.1 or 0.9 in partial/fully sheltered) retains more. Classify the roof's exposure honestly: a roof that is exposed today may be sheltered after adjacent development, and a "partially exposed" classification is the safe default where future shielding is possible. The thermal factor Ct captures building heat loss: a warm roof (heated building, Ct = 1.0) melts and sheds some snow, while a freezer building or an unheated structure (Ct up to 1.3) retains the full accumulation because nothing melts. The importance factor Is scales the load with Risk Category (0.8 for Category I, 1.0 for II, 1.1 for III, 1.2 for IV), so a hospital roof carries a higher design snow load than a storage building in the same location, and omitting Is understates the load for essential facilities.

### Apply the Slope Factor and Determine the Balanced Load

For sloped roofs, apply the roof slope factor Cs to convert the flat roof load to the sloped roof load ps, where Cs depends on the roof slope, the thermal condition, and the slipperiness of the surface (a slippery unobstructed warm roof sheds snow at lower slopes than a non-slippery cold roof). Confirm whether Cs permits a reduction at the actual slope, because the reduction thresholds differ for warm, cold, and slippery surfaces and a roof that does not meet the surface or unobstructed criteria cannot take the reduction. The balanced load is the uniform load the roof carries before any drift or sliding surcharge, and it must be applied to every roof area as the baseline; it is not the total load. A common error is to treat the balanced load as the final design load and stop, when in fact the balanced load is the platform on which drifts and sliding surcharges are then added.

### Evaluate Unbalanced, Drift, and Sliding Snow

Beyond the balanced load, ASCE 7 requires evaluation of unbalanced snow on gable and hip roofs (where wind clears the windward slope and deposits on the leeward), and of drift surcharges at roof steps, parapets, and changes in roof elevation. Leeward drifts form where snow blown off an upper roof deposits against a wall or onto a lower roof downwind; the drift height is computed from the upwind roof length (fetch) and pg, and the drift is a triangular surcharge superimposed on the balanced load. Windward drifts form against a wall or parapet on the roof itself from snow already on that roof, and the larger of the leeward and windward drifts governs at a given step. Sliding snow occurs where snow slides off a higher sloped roof onto a lower roof, delivering a concentrated surcharge over a defined width that can exceed the drift load; the sliding load must be evaluated separately and combined where both sliding and drift are possible. Confirm that every roof step, parapet, valley, and adjacency to a taller building has been checked for drift and sliding, because these local loads, not the uniform balanced load, drive most snow failures.

### Check Rain-On-Snow and Minimum Load Provisions

ASCE 7 requires a rain-on-snow surcharge of 5 psf added to the balanced load for low-slope roofs (slope ≤ W/50) in locations where pg ≤ 20 psf, because a rain event on a saturated snowpack adds weight that the snow load alone does not capture. Apply this surcharge only where the conditions (low slope, low ground snow, applicable climate) are met, and do not double-count it with the drift or sliding loads. Separately, confirm the minimum roof snow load for the jurisdiction, because some low-snow regions impose a minimum (often 20 to 30 psf) to guard against the variability of rare events, and the minimum can exceed the computed pf for a lightly loaded site. A roof designed only from the pf equation in a low-snow region can fall below the code minimum and must be checked against the floor.

## Common Traps

### The Balanced Load Treated As The Total Load

The engineer computes the flat and sloped roof load and applies it uniformly, then stops without evaluating drifts or sliding. The mechanism is that the balanced load looks complete and conservative, so the false signal of a finished calculation hides the local surcharges that govern; the harm is a roof that survives the uniform snow but collapses where a drift triples the load at a step or parapet the balanced load never represented.

### The Sheltered Roof Classified As Exposed

The roof is classified as fully exposed (Ce = 1.2) to reduce the load, but the site is sheltered by adjacent buildings or trees, or will be after development. The mechanism is that the exposure factor drops the balanced load, so the false signal of a "windy open site" hides a roof that actually retains snow; the harm is a balanced load understated by the exposure factor, on top of which the drifts will be even larger because the snow is not cleared by wind.

### The Drift Computed Without The Correct Fetch

The leeward drift is computed using the wrong upwind roof length, or the windward and leeward drifts are not both evaluated, so the drift height is understated. The mechanism is that drift size scales with fetch, so the false signal of a computed drift hides a larger drift the actual fetch would produce; the harm is a step or parapet designed for a small drift that the real fetch will overtop and overload.

### The Sliding Snow From The Upper Roof Ignored

A lower roof adjacent to a higher sloped roof is checked for wind drift but not for sliding snow, because sliding "seems unlikely." The mechanism is that sliding delivers a concentrated surcharge distinct from the drift, so the false signal of a drift check hides the sliding load; the harm is a lower roof designed for drift alone that is crushed by a sliding snow event the code required to be evaluated.

## Self-Check

- Is the ground snow load pg taken from the correct ASCE 7 map or a governing site-specific jurisdictional value, and not borrowed from a nearby unrelated location?
- Are the exposure factor Ce, thermal factor Ct, and importance factor Is assigned from the actual roof condition, building use, and Risk Category, with sheltered conditions not over-classified as exposed?
- Is the flat roof load pf computed as 0.7 Ce Ct Is pg, and is the sloped roof load reduced only where the slope factor Cs permits for the actual surface and thermal condition?
- Has every roof step, parapet, valley, and adjacency to a taller building been checked for leeward and windward drifts, with the drift height based on the correct upwind fetch?
- Has sliding snow from any higher sloped roof been evaluated and combined with the balanced and drift loads on the lower roof?
- Are the rain-on-snow surcharge and the jurisdictional minimum roof snow load both checked and applied where the conditions are met?
- Is the balanced load understood as the baseline on which drifts and sliding are superimposed, not as the final total design load?
