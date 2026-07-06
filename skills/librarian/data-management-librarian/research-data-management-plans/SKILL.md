---
name: research_data_management_plans.md
description: Use when the agent is helping a researcher write, review, or revise a data management plan (DMP) for a grant proposal, advising on data description and volume, file formats and organization, metadata standards, storage and backup during a project, sharing and long-term preservation, roles and responsibilities, ethics and privacy obligations for data, or aligning a DMP with funder requirements such as NSF, NIH, DOE, Horizon Europe, Wellcome Trust, or institutional policy.
---

# Research Data Management Plans

A data management plan, or DMP, is a living document that describes how research data will be handled across the entire lifecycle: created or collected, documented, stored and backed up during the project, analyzed, shared, preserved, and eventually retired. Funders increasingly require DMPs because the evidence behind published results has real value, and that value is destroyed when data is lost, undocumented, locked in obsolete formats, or kept on a failing laptop. A good DMP is not boilerplate; it is a project-specific plan that the researcher can and will actually follow. A bad DMP, one copied from a template with generic answers, satisfies the submission checkbox but produces no real data stewardship and often fails funder review.

The judgment problem is that researchers treat DMPs as administrative burden rather than as planning tools. They copy generic language, promise sharing they cannot deliver, name storage solutions they do not have, and ignore the real constraints of their data: sensitive human-subjects data, licensed third-party data, large volumes, proprietary formats, or ethical restrictions on sharing. Librarians and data support staff reviewing DMPs must catch these gaps before submission, because a DMP that over-promises becomes a compliance and integrity problem later, and a DMP that under-delivers undermines reproducibility and funder trust. The agent's job is to make the DMP specific, realistic, and aligned with both funder requirements and what the researcher can actually execute.

Use this skill when helping a researcher draft, review, or revise a DMP, advising on any section of a DMP, aligning with funder requirements, or evaluating whether a plan is realistic. The goal is to prevent the agent from producing generic boilerplate, promising unachievable sharing or preservation, ignoring sensitive data constraints, or treating the DMP as a one-time form rather than a living plan.

## Core Rules

### Treat The DMP As A Project-Specific Plan, Not Boilerplate

A DMP that could apply to any project applies to none. Every answer must reflect the specific data, methods, and constraints of the proposed research.

Specificity requirements:

- describe the actual data types: surveys, interviews, sensor streams, images, code, models, specimens;
- estimate realistic volumes and growth rates;
- identify the real formats, including proprietary ones, and address them honestly;
- name the specific storage, backup, and repository solutions to be used;
- ground the sharing plan in what is legally and ethically possible for this data.

When reviewing a DMP, flag any answer that reads as generic. Push the researcher to replace "we will store data securely" with the actual system, location, and backup approach.

### Map Every Section To Funder Requirements

Funders differ in what they require and how strictly they enforce it. A DMP must match the specific funder's expectations.

Funder alignment:

- identify the funder's DMP questions and any page or format limits;
- check whether the funder requires a specific template or tool, such as the DMPTool or DMPTuuli;
- note funder-specific expectations on sharing timing, repositories, and metadata;
- verify allowed costs for data management in the budget;
- confirm whether the funder requires a DMP at proposal stage, at award, or both, and whether updates are expected.

A technically excellent DMP that does not answer the funder's actual questions can be rejected. Align structure and content to the funder first.

### Describe Data Realistically, Including Volume And Formats

The data description anchors the whole plan. Vague or optimistic descriptions make the rest of the plan unverifiable.

Data description should cover:

- what data will be created, collected, or reused, and from what sources;
- expected volume at collection and after processing;
- file formats, distinguishing open and preservation formats from proprietary working formats;
- whether existing or third-party data is reused, and under what terms;
- how data will be organized, including folder structure and naming conventions;
- relationships among datasets, code, and supporting materials.

Honesty about volume and format matters because it determines storage costs, backup strategy, and repository feasibility. Underestimating volume is a common cause of later failure.

### Choose Formats With Preservation And Reuse In Mind

File formats determine whether data remains usable in five or ten years. The DMP should address format strategy.

Format guidance:

- prefer open, non-proprietary, and well-documented formats for preservation, such as CSV, TIFF, ODF, or netCDF;
- acknowledge proprietary working formats and plan to convert to open formats for sharing and preservation;
- avoid formats dependent on specific software versions or licenses where possible;
- document any transformations between working and preservation formats;
- consider community standards for the discipline, such as DICOM for medical imaging or FITS for astronomy.

Promising preservation in a format that will be obsolete is a hollow commitment. Match formats to long-term usability.

### Plan Storage And Backup For The Active Project

During the project, data is at greatest risk of loss. The DMP must describe concrete, sufficient storage and backup.

Active storage planning:

- name the actual storage location, such as institutional research storage, approved cloud, or a secure server;
- describe backup frequency, such as daily automated backups, and the backup location, ideally offsite;
- ensure backups follow the 3-2-1 principle: three copies, two media, one offsite;
- address version control for code and evolving datasets, using tools like Git where appropriate;
- describe access controls, especially for sensitive data;
- avoid naming personal devices, external hard drives, or USB sticks as primary storage.

"Laptop with Dropbox" is not a research-grade storage plan. Institutional or funder-approved solutions are expected.

### Address Metadata And Documentation Explicitly

Data without documentation is unusable by anyone else, including the researcher's future self. The DMP must commit to specific standards.

Documentation planning:

- name the metadata standard appropriate to the discipline, such as Dublin Core, DataCite, Darwin Core, or DDI;
- describe what documentation will accompany the data: readme files, codebooks, data dictionaries, protocols;
- plan for documentation of methods, variables, units, and codes;
- assign responsibility for creating and maintaining documentation;
- ensure documentation is sufficient for a qualified third party to understand and reuse the data.

Generic promises to "document the data" are insufficient. Name the standard and the artifacts.

### Make The Sharing Plan Realistic And Compliant

Sharing is where DMPs most often over-promise. The plan must be both ambitious and achievable.

Sharing considerations:

- identify a specific repository appropriate to the discipline and data type;
- confirm the repository accepts the data volume, format, and access conditions;
- determine the timing of sharing, aligning with funder expectations and publication;
- choose an appropriate license, typically a open license such as CC-BY or CC0 for data where permitted;
- address embargoes, if any, with justification;
- handle sensitive data through restricted-access sharing, de-identification, or controlled repositories rather than abandoning sharing.

A sharing plan that names no repository, or names one that will not accept the data, is not a plan. Verify feasibility.

### Handle Sensitive And Human-Subjects Data Carefully

Data from human subjects, health records, or vulnerable populations carries legal and ethical constraints that override default sharing assumptions.

Sensitive data planning:

- identify consent and IRB or ethics-board constraints on data sharing at the start;
- determine whether data can be de-identified or anonymized for sharing;
- plan for controlled-access repositories where direct sharing is not permitted;
- address legal obligations such as HIPAA, GDPR, or equivalent data protection law;
- describe data security during collection, storage, and analysis;
- avoid promising open sharing of data that consent or law forbids.

Over-promising sharing of sensitive data is an ethics violation. Under-sharing by default fails the spirit of open science. Find the legitimate path.

### Define Roles, Responsibilities, And Costs

A DMP without assigned responsibility is an aspiration. People and budget make it real.

Roles and costs:

- name who is responsible for data management, backup, documentation, and sharing;
- clarify responsibilities across collaborators, institutions, and students;
- identify data management costs and whether they are allowable under the funder's budget;
- plan for transitions when students or staff leave the project;
- address intellectual property and data ownership across collaborators.

Plans that assign no owner and no budget rarely get executed. Make responsibility and cost explicit.

### Treat The DMP As A Living Document

A DMP submitted at proposal time rarely survives contact with the actual project. Funders increasingly expect updates.

Living plan practices:

- review and update the DMP at major project milestones;
- revise when methods, data sources, or constraints change;
- document deviations from the original plan and the reasons;
- align the updated DMP with actual repository deposits at the end;
- retain version history for accountability.

A DMP frozen at submission is a fiction. Treat it as a plan that evolves with the research.

## Common Traps

### Copying Generic Boilerplate

A DMP that could describe any project describes none. Replace generic language with project-specific detail at every section.

### Over-Promising Sharing

Promising open sharing of data that is sensitive, licensed, or too large for any repository creates a compliance problem. Match the sharing plan to what is legally and practically possible.

### Naming Unrealistic Storage

Laptops, USB drives, and personal cloud accounts are not research-grade storage. Use institutional or funder-approved solutions with real backup.

### Ignoring Sensitive Data Constraints

Default open-sharing assumptions violate consent and law for human-subjects data. Identify constraints early and plan controlled-access alternatives.

### Vague Metadata Commitments

Promising to "document the data" without naming a standard or artifact leaves documentation undone. Specify the metadata standard and deliverables.

### No Assigned Responsibility Or Budget

A plan with no owner and no cost is an aspiration. Name responsible parties and budget for data management.

### Treating The DMP As A One-Time Form

Funders expect DMPs to evolve. A plan frozen at submission becomes inaccurate. Review and update at milestones.

### Misaligning With Funder Requirements

A strong DMP that ignores the funder's specific questions or template can be rejected. Align structure and content to the funder first.

## Self-Check

- Is every section of the DMP specific to the actual data, methods, and constraints of the project, not generic boilerplate?
- Does the plan align with the specific funder's questions, template, and sharing expectations?
- Are data types, volumes, and formats described realistically, including proprietary formats and conversion plans?
- Is active-project storage concrete, with named systems, backup frequency, offsite copies, and access controls, not personal devices?
- Is a specific metadata standard named, with planned documentation artifacts such as readmes, codebooks, and data dictionaries?
- Is the sharing plan feasible, naming a specific repository that will accept the data, with an appropriate license and timing?
- Are sensitive, human-subjects, or licensed data handled through de-identification or controlled-access repositories rather than default open sharing?
- Are roles, responsibilities, and data management costs assigned explicitly, including transitions when personnel leave?
- Is the DMP treated as a living document with planned updates at milestones and documentation of deviations?
- Has the plan been reviewed for over-promises that the researcher cannot realistically deliver?
