---
id: doc-1
title: MCTP 1.3.3 Specification
type: other
created_date: '2025-07-07'
---

| 2 | Document Identifier: DSP0236 |
|---|------------------------------|
| 3 | Date: 2024-03-25             |
| 4 | Version: 1.3.3               |
|   |                              |

# **Management Component Transport Protocol**

# **(MCTP) Base Specification**

**Includes MCTP Control Specifications**

**Supersedes: 1.3.2**

- **Document Class: Normative**
- **Document Status: Published**
- **Document Language: en-US**

## Copyright Notice

Copyright © 2009, 2010, 2013, 2014, 2016, 2019, 2024 DMTF. All rights reserved.

 DMTF is a not-for-profit association of industry members dedicated to promoting enterprise and systems management and interoperability. Members and non-members may reproduce DMTF specifications and documents for uses consistent with this purpose, provided that correct attribution is given. As DMTF specifications may be revised from time to time, the particular version and release date should always be noted.

 Implementation of certain elements of this standard or proposed standard may be subject to third-party patent rights, including provisional patent rights (herein "patent rights"). DMTF makes no representations to users of the standard as to the existence of such rights and is not responsible to recognize, disclose, or identify any or all such third-party patent right owners or claimants, nor for any incomplete or inaccurate identification or disclosure of such rights, owners, or claimants. DMTF shall have no liability to any party,

in any manner or circumstance, under any legal theory whatsoever, for failure to recognize, disclose, or

identify any such third-party patent rights, or for such party's reliance on the standard or incorporation

thereof in its products, protocols, or testing procedures. DMTF shall have no liability to any party

implementing such standards, whether such implementation is foreseeable or not, nor to any patent

owner or claimant, and shall have no liability or responsibility for costs or losses incurred if a standard is

withdrawn or modified after publication, and shall be indemnified and held harmless by any party

implementing the standard from any and all claims of infringement by a patent owner for such

implementations.

For information about patents held by third-parties which have notified DMTF that, in their opinion, such

patents may relate to or impact implementations of DMTF standards, visit

[https://www.dmtf.org/about/policies/disclosures.](https://www.dmtf.org/about/policies/disclosures)

 PCI-SIG, PCIe, and the PCI HOT PLUG design mark are registered trademarks or service marks of PCI-SIG.

All other marks and brands are the property of their respective owners.

This document's normative language is English. Translation into other languages is permitted.

| 40       |    | Foreword  7    |                                                                               |  |  |  |  |  |  |
|----------|----|----------------|-------------------------------------------------------------------------------|--|--|--|--|--|--|
| 41       |    | Introduction 8 |                                                                               |  |  |  |  |  |  |
| 42       | 1  |                | Scope  9                                                                      |  |  |  |  |  |  |
| 43       | 2  |                | Normative references  9                                                       |  |  |  |  |  |  |
| 44       |    | 2.1            | Approved references 9                                                         |  |  |  |  |  |  |
| 45       |    | 2.2            | Other references  9                                                           |  |  |  |  |  |  |
| 46       | 3  |                | Terms and definitions  10                                                     |  |  |  |  |  |  |
| 47       |    | 3.1            | Requirement term definitions 10                                               |  |  |  |  |  |  |
| 48       |    | 3.2            | MCTP term definitions  12                                                     |  |  |  |  |  |  |
| 49       | 4  |                | Symbols and abbreviated terms 18                                              |  |  |  |  |  |  |
| 50       | 5  |                | Conventions  20                                                               |  |  |  |  |  |  |
| 51       |    | 5.1            | Overview  20                                                                  |  |  |  |  |  |  |
| 52       |    | 5.2            | Byte ordering 20                                                              |  |  |  |  |  |  |
| 53       |    | 5.3            | Reserved fields  20                                                           |  |  |  |  |  |  |
| 54       | 6  |                | Management component relationships  21                                        |  |  |  |  |  |  |
| 55       | 7  |                | MCTP overview  21                                                             |  |  |  |  |  |  |
| 56       | 8  |                | MCTP base protocol 24                                                         |  |  |  |  |  |  |
| 57       |    | 8.1            | Overview  24                                                                  |  |  |  |  |  |  |
| 58       |    | 8.2            | MCTP packet fields 24                                                         |  |  |  |  |  |  |
| 59       |    | 8.3            | Special endpoint IDs  26                                                      |  |  |  |  |  |  |
| 60       |    | 8.4            | Packet payload and transmission unit sizes 27                                 |  |  |  |  |  |  |
| 61       |    | 8.5            | Maximum message body sizes 27                                                 |  |  |  |  |  |  |
| 62       |    | 8.6            | Message assembly  28                                                          |  |  |  |  |  |  |
| 63       |    | 8.7            | Dropped packets 28                                                            |  |  |  |  |  |  |
| 64       |    | 8.8            | Starting message assembly 29                                                  |  |  |  |  |  |  |
| 65       |    | 8.9            | Terminating message assembly/dropped messages  29                             |  |  |  |  |  |  |
| 66       |    | 8.10           | Dropped messages 30                                                           |  |  |  |  |  |  |
| 67       |    | 8.11           | MCTP versioning and message type support 30                                   |  |  |  |  |  |  |
| 68       |    | 8.12           | MCTP message types 31                                                         |  |  |  |  |  |  |
| 69       |    | 8.13           | Security  31                                                                  |  |  |  |  |  |  |
| 70       |    | 8.14           | Limitations 31                                                                |  |  |  |  |  |  |
| 71<br>72 |    | 8.15<br>8.16   | MCTP discovery and addressing 32<br>Devices with multiple media interfaces 33 |  |  |  |  |  |  |
| 73       |    | 8.17           | Peer transactions  33                                                         |  |  |  |  |  |  |
| 74       |    | 8.18           | Endpoint ID assignment and endpoint ID pools  34                              |  |  |  |  |  |  |
| 75       |    | 8.19           | Handling reassigned EIDs  39                                                  |  |  |  |  |  |  |
| 76       | 9  |                | MCTP bridging  39                                                             |  |  |  |  |  |  |
| 77       |    | 9.2            | Bridge and routing table examples  48                                         |  |  |  |  |  |  |
| 78       |    | 9.3            | Endpoint ID resolution 52                                                     |  |  |  |  |  |  |
| 79       |    | 9.4            | Bridge and bus owner implementation recommendations 54                        |  |  |  |  |  |  |
| 80       |    | 9.5            | Path and transmission unit discovery  55                                      |  |  |  |  |  |  |
| 81       |    | 9.6            | Path transmission unit requirements for bridges  58                           |  |  |  |  |  |  |
| 82       | 10 |                | Rate Limiting  59                                                             |  |  |  |  |  |  |
| 83       | 11 |                | MCTP control protocol 62                                                      |  |  |  |  |  |  |
| 84       |    | 11.2           | Terminology  63                                                               |  |  |  |  |  |  |
| 85       |    | 11.3           | Control message classes 63                                                    |  |  |  |  |  |  |
| 86       |    | 11.4           | MCTP control message format  64                                               |  |  |  |  |  |  |
| 87       |    | 11.5           | MCTP control message fields 64                                                |  |  |  |  |  |  |
| 88       |    | 11.6           | MCTP control message transmission unit size 65                                |  |  |  |  |  |  |
| 89       |    | 11.7           | Tag Owner (TO), Request (Rq), and Datagram (D) bit usage 65                   |  |  |  |  |  |  |
| 90       |    | 11.8           | Concurrent command processing 66                                              |  |  |  |  |  |  |

| 91  | 12 |      | MCTP control messages  67                                   |  |
|-----|----|------|-------------------------------------------------------------|--|
| 92  |    | 12.1 | Overview  67                                                |  |
| 93  |    | 12.2 | MCTP control message command codes 67                       |  |
| 94  |    | 12.3 | MCTP control message completion codes  69                   |  |
| 95  |    | 12.4 | Set Endpoint ID 70                                          |  |
| 96  |    | 12.5 | Get Endpoint ID  72                                         |  |
| 97  |    | 12.6 | Get Endpoint UUID  73                                       |  |
| 98  |    | 12.7 | Get MCTP version support  74                                |  |
| 99  |    | 12.8 | Get Message Type Support 77                                 |  |
| 100 |    | 12.9 | Get Vendor Defined Message Support 77                       |  |
| 101 |    |      | 12.10 Resolve Endpoint ID  79                               |  |
| 102 |    |      | 12.11 Allocate Endpoint IDs 80                              |  |
| 103 |    |      | 12.12 Routing Information Update 82                         |  |
| 104 |    |      | 12.13 Get Routing Table Entries  84                         |  |
| 105 |    |      | 12.14 Prepare for Endpoint Discovery 85                     |  |
| 106 |    |      | 12.15 Endpoint Discovery  86                                |  |
| 107 |    |      | 12.16 Discovery Notify  86                                  |  |
| 108 |    |      | 12.17 Get Network ID  87                                    |  |
| 109 |    |      | 12.18 Query Hop 87                                          |  |
| 110 |    |      | 12.19 Resolve UUID  88                                      |  |
| 111 |    |      | 12.20 Query rate limit 89                                   |  |
| 112 |    |      | 12.21 Request TX rate limit  90                             |  |
| 113 |    |      | 12.22 Update rate limit 91                                  |  |
| 114 |    |      | 12.23 Query Supported Interfaces 91                         |  |
| 115 |    |      | 12.24 Transport Specific  92                                |  |
| 116 | 13 |      | Vendor Defined – PCI and Vendor Defined – IANA messages  92 |  |
| 117 |    | 13.1 | Vendor Defined – PCI message format 94                      |  |
| 118 |    | 13.2 | Vendor Defined – IANA message format 94                     |  |
| 119 |    |      | ANNEX A (informative) Notation 95                           |  |
| 120 |    |      | ANNEX B (informative) Change log 96                         |  |
|     |    |      |                                                             |  |

| 123 | Figure 1 – Management component relationships 21                      |  |
|-----|-----------------------------------------------------------------------|--|
| 124 | Figure 2 – MCTP networks  22                                          |  |
| 125 | Figure 3 – MCTP topology  23                                          |  |
| 126 | Figure 4 – Generic message fields  24                                 |  |
| 127 | Figure 5 – Topmost bus owners  35                                     |  |
| 128 | Figure 6 – Split bridge  35                                           |  |
| 129 | Figure 7 – Acceptable failover/redundant communication topologies  40 |  |
| 130 | Figure 8 – Routing/bridging restrictions  41                          |  |
| 131 | Figure 9 – EID options for MCTP bridges  42                           |  |
| 132 | Figure 10 – Basic routing table entry fields 45                       |  |
| 133 | Figure 11 – Routing table population  46                              |  |
| 134 | Figure 12 – Example 1 Routing topology 48                             |  |
| 135 | Figure 13 – Example 2 Routing topology 50                             |  |
| 136 | Figure 14 – Example 3 Routing topology 51                             |  |
| 137 | Figure 15 – Endpoint ID resolution  53                                |  |
| 138 | Figure 16 – Resolving multiple paths 54                               |  |
| 139 | Figure 17 – Example path routing topology  56                         |  |

## **DSP0236 MCTP Base Specification**

| 140 | Figure 18 – Path transmission unit discovery flowchart 58                                |  |
|-----|------------------------------------------------------------------------------------------|--|
| 141 | Figure 19 – Example rate limiting message exchanges  60                                  |  |
| 142 | Figure 20 – MCTP control message format  64                                              |  |
| 143 | Figure 21 – Structure of Vendor ID field for Get Vendor Defined capabilities message  78 |  |
| 144 | Figure 22 – EID Pools from multiple bus owners 81                                        |  |
|     |                                                                                          |  |

## **Tables**

| 146 | Table 1 – MCTP base protocol common fields  24                        |  |
|-----|-----------------------------------------------------------------------|--|
| 147 | Table 2 – Special endpoint IDs  26                                    |  |
| 148 | Table 3 – MCTP Message Types Used in this Specification  31           |  |
| 149 | Table 4 – Example 1 Routing table for D2 49                           |  |
| 150 | Table 5 – Example 2 Routing table for D1 50                           |  |
| 151 | Table 6 – Example 3 Routing table for D2 51                           |  |
| 152 | Table 7 – Additional information tracked by bridges 52                |  |
| 153 | Table 8 – MCTP control protocol terminology 63                        |  |
| 154 | Table 9 – MCTP control message types 63                               |  |
| 155 | Table 10 – MCTP control message fields  64                            |  |
| 156 | Table 11 – Tag Owner (TO), Request (Rq) and Datagram (D) bit usage 65 |  |
| 157 | Table 12 – MCTP control command numbers  67                           |  |
| 158 | Table 13 – MCTP control message completion codes  69                  |  |
| 159 | Table 14 – Set Endpoint ID message  70                                |  |
| 160 | Table 15 – Get Endpoint ID message 72                                 |  |
| 161 | Table 16 – Get Endpoint UUID message format  73                       |  |
| 162 | Table 17 – Example UUID format  73                                    |  |
| 163 | Table 18 – Get MCTP version support message 74                        |  |
| 164 | Table 19 – Get Message Type Support message  77                       |  |
| 165 | Table 20 – Get Vendor Defined Message Support message  78             |  |
| 166 | Table 21 – Vendor ID formats 79                                       |  |
| 167 | Table 22 – Resolve Endpoint ID message 79                             |  |
| 168 | Table 23 – Allocate Endpoint IDs message 81                           |  |
| 169 | Table 24 – Routing Information Update message  83                     |  |
| 170 | Table 25 – Routing Information Update entry format 83                 |  |
| 171 | Table 26 – Get Routing Table Entries message 84                       |  |
| 172 | Table 27 – Routing Table Entry format  84                             |  |
| 173 | Table 28 – Prepare for Endpoint Discovery message  86                 |  |
| 174 | Table 29 – Endpoint Discovery message  86                             |  |
| 175 | Table 30 – Discovery Notify message  87                               |  |
| 176 | Table 31 – Get Network ID message format 87                           |  |
| 177 | Table 32 – Query Hop message  88                                      |  |
| 178 | Table 33 – Resolve UUID message 89                                    |  |
| 179 | Table 34 – Resolve UUID message entry format  89                      |  |
| 180 | Table 35 – Query rate limit message  89                               |  |
| 181 | Table 36 – Request TX rate limit message 90                           |  |
| 182 | Table 37 – Update rate limit message  91                              |  |
| 183 | Table 38 – Query Supported Interfaces 92                              |  |
| 184 | Table 39 – Transport Specific message  92                             |  |

| 185 | Table 40 – Vendor Defined – PCI message format  94 |  |
|-----|----------------------------------------------------|--|
| 186 | Table 41 – Vendor Defined – IANA message format 94 |  |
|     |                                                    |  |

<span id="page-6-0"></span>

| 188        | Foreword                                                                                                                                   |
|------------|--------------------------------------------------------------------------------------------------------------------------------------------|
| 189<br>190 | The Management Component Transport Protocol (MCTP) Base Specification (DSP0236) was prepared<br>by the PMCI Working Group.                 |
| 191<br>192 | DMTF is a not-for-profit association of industry members dedicated to promoting enterprise and systems<br>management and interoperability. |
| 193        | This version supersedes version 1.3.2. For a list of changes, see the change log in ANNEX B.                                               |
| 194        | Acknowledgments                                                                                                                            |
| 195        | DMTF acknowledges the following individuals for their contributions to this document:                                                      |
| 196        | Editor:                                                                                                                                    |
| 197        | •<br>Yuval Itkin – NVIDIA Corporation                                                                                                      |
| 198        | Contributors:                                                                                                                              |
| 199        | •<br>Alan Berenbaum – SMSC                                                                                                                 |
| 200        | •<br>Patrick Caporale – Lenovo                                                                                                             |
| 201        | •<br>Phil Chidester – Dell Inc                                                                                                             |
| 202        | •<br>Ira Kalman – Intel Corporation                                                                                                        |
| 203        | •<br>Edward Klodnicki – IBM                                                                                                                |
| 204        | •<br>Joe Kozlowski – Dell Inc                                                                                                              |
| 205        | •<br>Patrick Kutch – Intel Corporation                                                                                                     |
| 206        | •<br>John Leung – Intel Corporation                                                                                                        |
| 207        | •<br>Eliel Louzoun – Intel Corporation                                                                                                     |
| 208        | •<br>Patrick Schoeller – Hewlett Packard Enterprise                                                                                        |
| 209        | •<br>Hemal Shah – Broadcom Limited                                                                                                         |
| 210        | •<br>Tom Slaight – Intel Corporation                                                                                                       |
| 211        | •<br>Bob Stevens – Dell Inc.                                                                                                               |

## <span id="page-7-0"></span>Introduction

- The Management Component Transport Protocol (MCTP) defines a communication model intended to facilitate communication between:
- Management controllers and other management controllers
- Management controllers and managed devices
- The communication model includes a message format, transport description, message exchange patterns, and configuration and initialization messages.
- MCTP is designed so that it can potentially be used on many bus types. The protocol is intended to be used for intercommunication between elements of platform management subsystems used in computer systems, and is suitable for use in mobile, desktop, workstation, and server platforms. Management controllers such as a baseboard management controller (BMC) can use this protocol for communication between one another, as well as for accessing managed devices within the platform.
- Management controllers can use this protocol to send and receive MCTP-formatted messages across the
- different bus types that are used to access managed devices and other management controllers.

 Managed devices in a system need to provide an implementation of the message format to facilitate actions performed by management controllers.

It is intended that different types of devices in a management system might need to implement different

- portions of the complete capabilities defined by this protocol. Where relevant, this is called out in the individual requirements.

## <span id="page-8-0"></span>**1 Scope**

The *MCTP Base Specification* describes the command protocol, requirements, and use cases of a

- transport protocol for communication between discrete management controllers on a platform, as well as
- between management controllers and the devices they manage.
- This document is intended to meet the following objectives:
- Describe the MCTP Base transport protocol.
- Describe the MCTP control message protocol.

 The MCTP specifies a transport protocol format. This protocol is independent of the underlying physical bus properties, as well as the "data-link" layer messaging used on the bus. The physical and data-link layer methods for MCTP communication across a given medium are defined by companion "transport binding" specifications, such as [DSP0238,](#page-8-4) MCTP over PCIe® Vendor Defined Messaging, and [DSP0237,](#page-8-5) MCTP over SMBus/I 2C. This approach enables future transport bindings to be defined to support additional buses such as USB, RMII, and others, without affecting the base MCTP specification.

## <span id="page-8-1"></span>**2 Normative references**

 The following referenced documents are indispensable for the application of this document. For dated references, only the edition cited applies. For undated references, the latest edition of the referenced document (including any amendments) applies.

## <span id="page-8-2"></span>**2.1 Approved references**

- <span id="page-8-5"></span>DMTF DSP0237, Management Component Transport Protocol SMBus/I2C Transport Binding
- Specification
- <span id="page-8-4"></span>[https://www.dmtf.org/standards/published\\_documents/DSP0237\\_1.0.x.pdf](https://www.dmtf.org/dsp/DSP0237)
- DMTF DSP0238, Management Component Transport Protocol (MCTP) PCIe VDM Transport Binding Specification
- <span id="page-8-9"></span>[https://www.dmtf.org/standards/published\\_documents/DSP0238\\_1.0.x.pdf](https://www.dmtf.org/dsp/DSP0238)
- DMTF, [DSP0239,](http://www.dmtf.org/standards/published_documents/) *Management Component Transport Protocol (MCTP) IDs and Codes* [https://www.dmtf.org/standards/published\\_documents/DSP0239\\_1.3.x.pdf](https://www.dmtf.org/dsp/DSP0239)
- <span id="page-8-8"></span> DMTF DSP2016, Management Component Transport Protocol (MCTP) Overview White Paper [https://www.dmtf.org/standards/published\\_documents/DSP2016\\_1.0.pdf](https://www.dmtf.org/standards/published_documents/DSP2016_1.0.pdf)
- <span id="page-8-11"></span>DMTF [DSP4004,](http://www.dmtf.org/standards/published_documents/DSP4004.pdf) *DMTF Release Process v2.7*
- <span id="page-8-3"></span>[https://www.dmtf.org/standards/published\\_documents/DSP4004\\_2.7.pdf](https://www.dmtf.org/standards/published_documents/DSP4004_2.7.pdf)

## **2.2 Other references**

- Hewlett-Packard, Intel, Microsoft, Phoenix, and Toshiba, *Advanced Configuration and Power Interface Specification v5.0*, ACPI, December 6, 2011
- <span id="page-8-10"></span>[http://www.acpi.info/downloads/ACPIspec50.pdf](http://www.acpi.info/DOWNLOADS/ACPIspec50.pdf)
- <span id="page-8-12"></span> IETF, RFC20, *ASCII format for Network Interchange*, October 16, 1969 <https://datatracker.ietf.org/doc/html/rfc20>
- <span id="page-8-6"></span> IETF, [RFC2119,](http://www.ietf.org/rfc/rfc2119.txt) *Key Words for use in RFCs to Indicate Requirement Levels*, March 1997 <https://datatracker.ietf.org/doc/rfc2119/>
- <span id="page-8-7"></span>IETF, RFC4122, *A Universally Unique Identifier (UUID) URN Namespace*, July 2005
- <https://datatracker.ietf.org/doc/rfc4122/>

- Intel, Hewlett-Packard, NEC, and Dell, *Intelligent Platform Management Interface Specification: Second*
- *Generation v2.0*, IPMI, 2004
- [https://web.archive.org/web/20131213024533if\\_/http://www.intel.com/content/dam/www/public/us/en/doc](https://web.archive.org/web/20131213024533if_/http:/www.intel.com/content/dam/www/public/us/en/documents/product-briefs/second-gen-interface-spec-v2-rev1-4.pdf)
- [uments/product-briefs/second-gen-interface-spec-v2-rev1-4.pdf](https://web.archive.org/web/20131213024533if_/http:/www.intel.com/content/dam/www/public/us/en/documents/product-briefs/second-gen-interface-spec-v2-rev1-4.pdf)
- <span id="page-9-2"></span> ISO/IEC Directives, Part 2, *Principles and rules for the structure and drafting of ISO and IEC documents* <https://www.iso.org/sites/directives/current/part2/index.xhtml>
- <span id="page-9-4"></span>NXP Semiconductors, *UM10204 I2C-bus specification and user manual,* Rev. 5, October 9, 2012
- [https://web.archive.org/web/20170625174556if\\_/http://www.cs.columbia.edu/~cs4823/handouts/UM1020](https://web.archive.org/web/20170625174556if_/http:/www.cs.columbia.edu/%7Ecs4823/handouts/UM10204.pdf) [4.pdf](https://web.archive.org/web/20170625174556if_/http:/www.cs.columbia.edu/%7Ecs4823/handouts/UM10204.pdf)
- PCI-SIG, PCI Express™ Specifications
- [https://www.pcisig.com/specifications/pciexpress/](http://www.pcisig.com/specifications/pciexpress/)
- <span id="page-9-3"></span> SMBus, *System Management Bus (SMBus) Specification v2.0*, SMBus, 2000 <http://www.smbus.org/specs/smbus20.pdf>

## <span id="page-9-0"></span>**3 Terms and definitions**

- In this document, some terms have a specific meaning beyond the normal English meaning. Those terms are defined in this clause.
- The terms "shall" ("required"), "shall not", "should" ("recommended"), "should not" ("not recommended"), "may", "need not" ("not required"), "can" and "cannot" in this document are to be interpreted as described in [ISO/IEC Directives, Part 2,](#page-9-2) Clause 7. The terms in parentheses are alternatives for the preceding term, for use in exceptional cases when the preceding term cannot be used for linguistic reasons. Note that [ISO/IEC Directives, Part 2,](#page-9-2) Clause 7 specifies additional alternatives. Occurrences of such additional alternatives shall be interpreted in their normal English meaning.
- The terms "clause", "subclause", "paragraph", and "annex" in this document are to be interpreted as described in [ISO/IEC Directives, Part 2,](#page-9-2) Clause 6.
- The terms "normative" and "informative" in this document are to be interpreted as described in [ISO/IEC](#page-9-2)  [Directives, Part 2,](#page-9-2) Clause 3. In this document, clauses, subclauses, or annexes labeled "(informative)" do not contain normative content. Notes and examples are always informative elements.

## <span id="page-9-1"></span>**3.1 Requirement term definitions**

- This clause defines key phrases and words that denote requirement levels in this specification. These definitions are consistent with the terms defined in [RFC2119.](#page-8-6)
- **3.1.1**
- **can**
- used for statements of possibility and capability, whether material, physical, or causal
- **3.1.2**
- **cannot**
- used for statements of possibility and capability, whether material, physical or causal
- **3.1.3**
- **conditional**
- indicates requirements to be followed strictly to conform to the document when the specified conditions
- are met.

**3.1.4**

## **deprecated**

- indicates that an element or profile behavior has been outdated by newer constructs.
- **3.1.5**
- **mandatory**
- indicates requirements to be followed strictly to conform to the document and from which no deviation is
- permitted.
- **3.1.6**
- **may**
- indicates a course of action permissible within the limits of the document.

Note 1 to entry: An implementation that does *not* include a particular option shall be prepared to interoperate with

- another implementation that *does* include the option, although perhaps with reduced functionality. An implementation
- that *does* include a particular option shall be prepared to interoperate with another implementation that does *not* include the option (except for the feature that the option provides).
- **3.1.7**
- **may not**
- indicates flexibility of choice with no implied preference
- **3.1.8**
- **need not**
- indicates a course of action permissible within the limits of the document.
- **3.1.9**

#### **not recommended**

- indicates that valid reasons may exist in particular circumstances when the particular behavior is
- acceptable or even useful, but the full implications should be understood and carefully weighed before implementing any behavior described with this label.
- **3.1.10**
- **obsolete**
- indicates that an item was defined in prior specifications but has been removed from this specification.

## **3.1.11**

- **optional**
- <span id="page-10-0"></span>indicates a course of action permissible within the limits of the document.

### **3.1.12**

- **physical layer**
- the electrical bus of the MCTP network.

#### **3.1.13**

#### **recommended**

- indicates that valid reasons may exist in particular circumstances to ignore a particular item, but the full
- implications should be understood and carefully weighed before choosing a different course.

- **3.1.14**
- **required**
- indicates that the item is an absolute requirement of the specification.
- **3.1.15**
- **shall**
- indicates that the item is an absolute requirement of the specification.
- **3.1.16**
- **shall not**
- indicates that the definition is an absolute prohibition of the specification.

## **3.1.17**

- **should**
- indicates that among several possibilities, one is recommended as particularly suitable, without
- mentioning or excluding others, or that a certain course of action is preferred but not necessarily required.

## **3.1.18**

- **should not**
- <span id="page-11-0"></span>indicates that a certain possibility or course of action is deprecated but not prohibited.

## **3.2 MCTP term definitions**

- For the purposes of this document, the following terms and definitions apply.
- **Address Resolution Protocol**
- **ARP**
- refers to the procedure used to dynamically determine the addresses of devices on a shared
- communication medium.

### **baseline transmission unit**

- the required common denominator size of a transmission unit for packet payloads that are carried in an
- MCTP packet. Baseline Transmission Unit-sized packets are guaranteed to be routable within an MCTP network.

## **baseboard management controller**

- **BMC**
- a term coined by the IPMI specifications for the main management controller in an IPMI-based platform
- management subsystem. Also sometimes used as a generic name for a motherboard resident
- management controller that provides motherboard-specific hardware monitoring and control functions for
- the platform management subsystem.

## **binary-coded decimal**

**BCD**

indicates a particular binary encoding for decimal numbers where each four bits (*nibble*) in a binary

number is used to represent a single decimal digit, and with the least significant four bits of the binary

 number corresponding to the least significant decimal digit. The binary values 0000b through 1001b represent decimal values 0 through 9, respectively.

 EXAMPLE: BCD encoding a byte can represent a two-digit decimal number where the most significant nibble (bits 7:4) of the byte contains the encoding for the most significant decimal digit and the least

- significant nibble (bits 3:0) contains the encoding for the least significant decimal digit (for example,
- 0010\_1001b in BCD encoding corresponds to the decimal number 29).

#### **bridge**

- generically, the circuitry and logic that connects one computer bus or interconnect to another, allowing an agent on one to access the other. Within this document, the term *bridge* refers to MCTP bridge, unless
- otherwise indicated.

###

#### **burst**

 a number of consecutive baseline transmission unit Packets that the transmitter endpoint sends with minimal delay between those baseline transmission unit packets.

##

**bus**

 a physical addressing domain shared between one or more platform components that share a common physical layer address space.

####

#### **bus owner**

the party responsible for managing address assignments (can be logical or physical addresses) on a bus

(for example, in MCTP, the bus owner is the party responsible for managing EID assignments for a given

bus). A bus owner may also have additional media-specific responsibilities, such as assignment of

- physical addresses.
- **byte**
- an 8-bit quantity. Also referred to as an *octet*.
- Note 1 to entry: PMCI specifications use the term *byte*, not *octet*.
- **endpoint**
- see [MCTP endpoint](#page-14-0)
- **endpoint ID**
- **EID**
- see [MCTP endpoint ID](#page-14-1)

## **Globally Unique Identifier**

- **GUID**
- see [UUID](#page-17-1)

- **host interface**
- a hardware interface and associated protocols that is used by software running locally on the host processors to access the hardware of a management subsystem within a managed system.

##

- **Inter-Integrated Circuit**
- **I 2C**
- a multi-master, two-wire, serial bus originally developed by Philips Semiconductor; now maintained by NXP Semiconductors,
- **intelligent management device**

### **IMD**

- a management device that is typically implemented using a microcontroller and accessed through a
- messaging protocol. Management parameter access provided by an IMD is typically accomplished using
- an abstracted interface and data model rather than through direct "register level" accesses.
- **Intelligent Platform Management Interface**
- **IPMI**
- a set of specifications defining interfaces and protocols originally developed for server platform
- management by the IPMI Promoters Group: Intel, Dell, HP, and NEC
- **managed entity**
- the physical or logical entity that is being managed through management parameters.
- EXAMPLE 1: *physical* entities include fans, processors, power supplies, circuit cards, chassis, and so on.
- EXAMPLE 2: *logical* entities include virtual processors, cooling domains, system security states, and so on.
- **Management Component Transport Protocol**
- **MCTP**
- the protocol defined in this specification.

## **management controller**

- a microcontroller or processor that aggregates management parameters from one or more managed
- devices and makes access to those parameters available to local or remote software, or to other
- management controllers, through one or more management data models. Management controllers may
- also interpret and process management-related data, and initiate management-related actions on
- managed devices. While a native data model is defined for PMCI, it is designed to be capable of
- supporting other data models, such as CIM, IPMI, and vendor-specific data models. The microcontroller
- or processor that serves as a management controller can also incorporate the functions of a management

device.

## **managed device**

for this specification, managed device refers to a device that is typically implemented using a

microcontroller and accessed through a messaging protocol and is used for accessing one or more

management parameters. Management parameter access provided by a managed device is typically

accomplished using an abstracted interface and data model rather than through direct "register level"

accesses. A managed device responds to management requests, but does not initiate or aggregate

- management operations except in conjunction with a management controller (that is, it is a *satellite*
- device that is subsidiary to one or more management controllers).

###

#### **management parameter**

a particular datum representing a characteristic, capability, status, or control point associated with a

 managed entity. Example management parameters include temperature, speed, volts, on/off, link state, uncorrectable error count, device power state, and so on.

##

#### **MCTP bridge**

 an MCTP endpoint that can route MCTP messages not destined for itself that it receives on one interconnect onto another without interpreting them. The ingress and egress media at the bridge may be

either homogeneous or heterogeneous. Also referred to in this document as a "bridge".

###

### **MCTP bus owner**

responsible for EID assignment for MCTP or translation on the buses that it is a master of. The MCTP bus

 owner may also be responsible for physical address assignment. For example, for SMBus/I2C bus segments, the MCTP bus owner is also the ARP master. This means the bus owner assigns dynamic

SMBus/I2C addresses to those devices requiring it.

###

#### **MCTP control command**

commands defined under the MCTP *control* message type that are used for the initialization and

management of MCTP communications (for example, commands to assign EIDs, discover device MCTP

- capabilities, and so on)

## <span id="page-14-0"></span>**MCTP endpoint**

## an MCTP communication terminus

Note 1 to entry: An MCTP endpoint is a terminus or origin of MCTP packets or messages. That is, the

 combined functionality within a physical device that communicates using the MCTP transport protocol and handles MCTP control commands. This includes MCTP-capable management controllers and managed

devices.

## **Note 2 to entry: Also referred to in this document as "endpoint".**

## <span id="page-14-1"></span>**MCTP endpoint ID**

- the logical address used to route MCTP messages to a specific MCTP endpoint.
- Note 1 to entry: A numeric handle (logical address) that uniquely identifies a particular MCTP endpoint
- within a system for MCTP communication and message routing purposes. Endpoint IDs are unique
- among MCTP endpoints that comprise an MCTP communication network within a system. MCTP EIDs
- are only unique within a particular MCTP network. That is, they can be duplicated or overlap from one
- MCTP network to the next.

**Note 2 to entry: Also referred to in this document as "endpoint ID" and abbreviated "EID".**

## **MCTP host interface**

- a host interface that enables host software to locally access an MCTP Network in the managed system.

## **MCTP management controller**

- a management controller that is an MCTP endpoint
- Note 1 to entry: Unless otherwise indicated, the term "management controller" refers to an "MCTP
- management controller" in this document.

##

## **MCTP managed device**

- a managed device that is an MCTP endpoint
- Note 1 to entry: Unless otherwise indicated, the term "managed device" refers to an "MCTP managed device" in this document.

## <span id="page-15-0"></span>**MCTP message**

- a unit of communication based on the message type that is relayed through the MCTP Network using one or more MCTP packets.

## **MCTP network**

- a collection of MCTP endpoints that communicate using MCTP and share a common MCTP endpoint ID space.

### **MCTP network ID**

- a unique identifier to distinguish each independent MCTP network within a platform.

### <span id="page-15-1"></span>**MCTP packet**

- the unit of data transfer used for MCTP communication on a given physical medium.

### <span id="page-15-2"></span>**MCTP packet payload**

- refers to the portion of the message body of an MCTP message that is carried in a single MCTP packet.
- **message**
- see [MCTP message](#page-15-0)

### **message assembly**

- the process of receiving and linking together two or more MCTP packets that belong to a given MCTP message to allow the entire message header and message data (payload) to be extracted.

### **message body**

- the portion of an MCTP message that carries the message type field and any message type-specific data
- associated with the message.

- Note 1 to entry: An MCTP message spans multiple MCTP packets when the message body needs are
- larger than what can fit in a single MCTP packet. Thus, the message body portion of an MCTP message
- can span multiple MCTP packets.

#### **message disassembly**

 the process of taking an MCTP message where the message's header and data (payload) cannot be carried in a single MCTP packet and generating the sequence of two or more packets required to deliver that message content within the MCTP network.

#### **message originator**

- the original transmitter (source) of a message targeted to a particular message terminus.

#### **message terminus**

 the name for a triplet of fields called the MCTP Source Endpoint ID, Tag Owner bit value, and Message Tag value.

- Note 1 to entry: Together, these fields identify the packets for an MCTP message within an MCTP
- network for the purpose of message assembly. The message terminus itself can be thought of as
- identifying a set of resources within the recipient endpoint that is handling the assembly of a particular message.

## **most significant byte**

- **MSB**
- refers to the highest order byte in a number consisting of multiple bytes.
- **nibble**
- the computer term for a four-bit aggregation, or half of a byte
- **packet**
- see [MCTP packet](#page-15-1)
- **packet payload**
- see [MCTP packet payload](#page-15-2)
- **payload**
- refers to the information bearing fields of a message.
- Note 1 to entry: This is separate from those fields and elements that are used to transport the message
- from one point to another, such as address fields, framing bits, checksums, and so on. In some instances, a given field may be both a payload field and a transport field.

####

## <span id="page-16-0"></span>**physical transport binding**

- refers to specifications that define how the MCTP base protocol and MCTP control commands are
- implemented on a particular physical transport type and medium, such as SMBus/I 2C, PCI Express™
- Vendor Defined Messaging, and so on.

## **Platform Management Component Intercommunications**

- **PMCI**
- name for a working group under the Distributed Management Task Force's Pre-OS Workgroup that is
- chartered to define standardized communication protocols, low level data models, and transport
- definitions that support communications with and between management controllers and managed devices
- that form a platform management subsystem within a managed computer system.
- **Rate Limiting**
- a method for limiting the data rate sent from an MCTP endpoint to another MCTP endpoint.

## **Reduced Media Independent Interface**

- **RMII**
- a reduced signal count MAC to PHY interface, based on the IEEE Media Independent Interface (MII),
- which was specified by the RMII Consortium (3Com Corporation; AMD Inc.; Bay Networks, Inc.;
- Broadcom Corp.; National Semiconductor Corp.; and Texas Instruments Inc.)
- **simple endpoint**
- an MCTP endpoint that is not associated with either the functions of an MCTP bus owner or an MCTP bridge.

## **Transmission Unit**

 refers to the size of the portion of the MCTP packet payload, which is the portion of the message body carried in an MCTP packet.

- **transport binding**
- see [physical transport binding](#page-16-0)
- <span id="page-17-1"></span>**Universally Unique Identifier**
- **UUID**
- refers to an identifier originally standardized by the Open Software Foundation (OSF) as part of the Distributed Computing Environment (DCE)
- Note 1 to entry: UUIDs are created using a set of algorithms that enables them to be independently
- generated by different parties without requiring that the parties coordinate to ensure that generated IDs
- do not overlap. In this specification, [RFC4122](#page-8-7) is used as the base specification describing the format and generation of UUIDs.
- <span id="page-17-0"></span>Note 2 to entry: Also, sometimes referred to as a globally unique identifier (GUID).

## **4 Symbols and abbreviated terms**

- The following symbols and abbreviations are used in this document.
- **ACPI**
- Advanced Configuration and Power Interface

|     | DSP0236                             | MCTP Base Specification |
|-----|-------------------------------------|-------------------------|
| 641 |                                     |                         |
| 642 | ARP                                 |                         |
| 643 | Address Resolution Protocol         |                         |
| 644 |                                     |                         |
| 645 | BCD                                 |                         |
| 646 | binary-coded decimal                |                         |
| 647 |                                     |                         |
| 648 | BMC                                 |                         |
| 649 | baseboard management controller     |                         |
| 650 |                                     |                         |
| 651 | CIM                                 |                         |
| 652 | Common Information Model            |                         |
| 653 |                                     |                         |
| 654 | EID                                 |                         |
| 655 | endpoint identifier                 |                         |
| 656 |                                     |                         |
| 657 | FIFO                                |                         |
| 658 | first-in first-out                  |                         |
| 659 |                                     |                         |
| 660 | GUID                                |                         |
| 661 | Globally Unique Identifier          |                         |
| 662 |                                     |                         |
|     | I 663 2C                            |                         |
| 664 | Inter-Integrated Circuit            |                         |
| 665 |                                     |                         |
| 666 | IANA                                |                         |
| 667 | Internet Assigned Numbers Authority |                         |
|     |                                     |                         |

- **IP**
- Internet Protocol
- **IPMI**
- Intelligent platform management interface
- **ISO/IEC**
- International Organization for Standardization/International Engineering Consortium
- **MCTP**
- Management Component Transport Protocol

## **MCTP Base Specification DSP0236**

| 680<br>681<br>682 | MSB<br>most significant byte                              |
|-------------------|-----------------------------------------------------------|
| 683<br>684<br>685 | PCIe<br>Peripheral Component Interconnect (PCI) Express   |
| 686<br>687<br>688 | PMCI<br>Platform Management Component Intercommunications |
| 689<br>690<br>691 | RMII<br>Reduced Media Independent Interface               |
| 692<br>693<br>694 | SMBus<br>System Management Bus                            |
| 695<br>696<br>697 | TCP/IP<br>Transmission Control Protocol/Internet Protocol |
| 698<br>699<br>700 | USB<br>Universal Serial Bus                               |
| 701<br>702<br>703 | UUID<br>Universally Unique Identifier                     |
| 704<br>705<br>706 | VDM<br>Vendor Defined Message                             |
|                   |                                                           |

# <span id="page-19-1"></span><span id="page-19-0"></span>**5 Conventions**

## **5.1 Overview**

<span id="page-19-2"></span>The conventions described in the following clauses apply to this specification.

## **5.2 Byte ordering**

 Unless otherwise specified, byte ordering of multi-byte numeric fields or bit fields is "Big Endian" (that is, the lower byte offset holds the most significant byte, and higher offsets hold lesser significant bytes).

## <span id="page-19-3"></span>**5.3 Reserved fields**

 Unless otherwise specified, any reserved, unspecified, or unassigned values in enumerations or other numeric ranges are reserved for future definition by DMTF.

- 716 Unless otherwise specified, numeric or bit fields that are designated as reserved shall be written as 0
- <span id="page-20-0"></span>717 (zero) and ignored when read.

## 718 **6 Management component relationships**

- 719 [Figure 1](#page-20-2) illustrates the relationship between devices, management controllers, managed devices, and
- 720 managed entities, which are described in Clause [3.2.](#page-11-0)

721

<span id="page-20-2"></span>722 **Figure 1 – Management component relationships**

## <span id="page-20-1"></span>723 **7 MCTP overview**

724 This clause provides an overview of the main elements of MCTP. Additional overview information is 725 available in the MCTP white paper, [DSP2016.](#page-8-8)

 MCTP is a transport independent protocol that is used for intercommunication within an MCTP Network. An MCTP Network that consists of one of more physical transports that are used to transfer MCTP Packets between MCTP Endpoints. MCTP Transport Binding Specifications define how the MCTP protocol is implemented across a particular physical transport medium. For example, DMTF has defined transport bindings for MCTP over [SMBus/](#page-9-3)I 730 [2C](#page-9-4) and MCTP over PCIe using PCIe Vendor Defined Messages (VDMs), and others.

- 732 An MCTP Endpoint is the terminus for MCTP communication. A physical device that supports MCTP may
- 733 provide one or more MCTP Endpoints. Endpoints are addressed using a logical address called the
- 734 Endpoint ID, or EID. EIDs in MCTP are analogous to IP Addresses in Internet Protocol networking. EIDs 735 can be statically or dynamically allocated.
- 736 A system implementation can contain multiple MCTP Networks. Each MCTP Network has its own 737 separate EID space. There is no coordination of EIDs between MCTP Networks. EIDs can overlap 738 between MCTP Networks.
- 739 An MCTP Network may provide an MCTP Network ID that can be used to differentiate different MCTP
- 740 Networks when more than one MCTP Network can be accessed by an entity such as system software.
- 741 The Network ID is also used when an entity has more than one point of access to the MCTP Network. In

## **MCTP Base Specification DSP0236**

- 742 this case, the MCTP Network ID enables the entity to tell whether the access points provide access to the
- 743 same MCTP Network or to different MCTP Networks.
- 744 The DMTF MCTP specifications also include the definition of transport bindings for MCTP host interfaces.
- 745 MCTP host interfaces are used by software that runs locally on the host processors of the managed 746 system to access an MCTP Network.
- 747

<span id="page-21-0"></span>748

749 **Figure 2 – MCTP networks**

750 [Figure 2](#page-21-0) shows the different ways MCTP Networks can exist in a system. In this example, Network A

751 connects a Management Controllers (MC) and managed devices (MD) on a motherboard with devices on

752 PCIe Card 1 using MCTP over PCIe Vendor Defined Messages. Note that there are two host interfaces

753 (host i/f) on standard PCIe (host software accessible) that can be used by host software to access this

754 particular network. This network thus requires an MCTP Network ID so that the host software can tell that

755 the two host interfaces connect to the same MCTP Network.

756 Network B represents a network that is solely used for interconnecting devices within PCIe Card 2. This 757 MCTP Network would typically not require an MCTP Network ID since it is not visible to host software or 758 any other entity that would needs to differentiate Network B from another MCTP Network in the system.

759 Network C represents an MCTP Network on an add-in module. This network is separate from networks A 760 and B but can accessed by host software through PCIe. Thus, this network requires a Network ID so that 761 host software can differentiate that Network C is a different network than Network A.

- 762 MCTP Messages are comprised of one or more MCTP Packets. MCTP defines fields that support the
- 763 assembly of received MCTP Packets into MCTP Messages and the disassembly of MCTP Messages into 764 packets for transmission.
- 765 MCTP is designed to be able to transfer multiple Message Types in an interleaved manner using the
- 766 same protocol. MCTP Message Types identified using a Message Type number. The use of the message
- 767 type number is similar to a well-known port number in Internet Protocol. It identifies MCTP Messages that
- 768 are all associated with a particular specification. This specification defines a Message Type for MCTP 769 Control Messages that are used to initialize and maintain the MCTP Network. DMTF has also defined

- 770 Message Types for use by the PMCI (Platform Management Communications Interconnect)
- 771 specifications, Vendor-specific Messaging over MCTP, and so on. MCTP Message Type number
- 772 assignments are provided in [DSP0239.](#page-8-9) [DSP0239](#page-8-9) will be updated as new message types are defined in
- 773 the future.
- 774 MCTP Control Messages use a request/response protocol. It is important to note that the base transport
- 775 protocol defined by MCTP just defines a protocol for the transport of MCTP messages. Whether the
- 776 message content is a request, a response, or something else is part of the particular Message Type
- 777 definition.
- 778 In MCTP, a Bus is defined as a physical medium that shares a single physical address space. MCTP
- 779 includes the definition of a function called the MCTP Bus Owner. The Bus Owner provides two main
- 780 functions: It distributes EIDs to Endpoints when the MCTP implementation uses EIDs that are dynamically
- 781 allocated, and it provides the way for an Endpoint to resolve an EID into the physical address used that is
- 782 required to deliver a message to the target Endpoint.
- 783 Busses can be interconnected within an MCTP Network using MCTP Bridges to forward MCTP packets
- 784 between busses. Bridges also handle the task of managing the difference in moving packets from one
- 785 type of physical media to another, such as moving an MCTP packet between SMBus/I2C and PCIe 786 Vendor Defined Messaging.
- 787 The following example illustrates how MCTP can be used within a hypothetical platform management 788 subsystem implementation. More complex topologies, with multi-levels of bridges and greater numbers of
- 789 busses and devices can be readily supported by MCTP as required.

<span id="page-22-0"></span>791 **Figure 3 – MCTP topology**

## <span id="page-23-0"></span>793 **8 MCTP base protocol**

## <span id="page-23-1"></span>794 **8.1 Overview**

795 The MCTP base protocol defines the common fields for MCTP packets and messages and their usage.

 Though there are medium-specific packet header fields and trailer fields, the fields for the base protocol are common for all media. These common fields support the routing and transport of messages between MCTP endpoints and the assembly and disassembly of large messages from and into multiple MCTP packets, respectively. The base protocol's common fields include a message type field that identifies what particular higher layer class of message is being carried using the MCTP base protocol.

## <span id="page-23-2"></span>801 **8.2 MCTP packet fields**

802 [Figure 4](#page-23-3) shows the fields that constitute a generic MCTP packet.

| Medium-specific<br>Header |                                                                       | Physical Medium-Specific Header<br>Includes physical source and destination addresses. |                                        |                       |             |             |                 |        |              |
|---------------------------|-----------------------------------------------------------------------|----------------------------------------------------------------------------------------|----------------------------------------|-----------------------|-------------|-------------|-----------------|--------|--------------|
| MCTP transport<br>header  | RSVD                                                                  | Hdr<br>version                                                                         | Destination<br>endpoint ID             | Source<br>endpoint ID | S<br>O<br>M | E<br>O<br>M | Pkt<br>Seq<br># | T<br>O | Msg<br>tag   |
| MCTP packet payload       | Message header<br>I<br>C                                              | Message type                                                                           | Message type-specific<br>Header fields |                       |             |             |                 |        | Message body |
|                           | Msg integrity check                                                   |                                                                                        |                                        |                       |             |             |                 |        |              |
| Medium-specific trailer   | Physical Medium-Specific Trailer (for example, data integrity fields) |                                                                                        |                                        |                       |             |             |                 |        |              |

803

## <span id="page-23-3"></span>804 **Figure 4 – Generic message fields**

805 [Table 1](#page-23-4) defines the base protocol common fields.

## 806 **Table 1 – MCTP base protocol common fields**

<span id="page-23-4"></span>

| Field Name                 | Field Size         | Description                                                                                                                                                                                                                                                                                                                                                                                                                                  |
|----------------------------|--------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Medium-specific<br>header  | see<br>description | This field represents the physical addressing and framing information that is<br>used for transferring MCTP packets between devices on a particular physical<br>medium. The size and type of any sub-fields or data within this field are<br>defined by the corresponding transport binding specification for MCTP<br>messaging on a given medium (for example, MCTP over SMBus/I2C, MCTP<br>over PCIe Vendor Defined Messaging, and so on). |
| Medium-specific<br>trailer | see<br>description | This field represents any additional medium-specific trailer fields (if any) that<br>are required for transferring MCTP packets between devices on a particular<br>physical medium. A typical use of this field would be to hold per-packet data<br>integrity fields (for example CRC, checksum, and so on) that would be<br>specified for the particular medium.                                                                            |

| Field Name               | Field Size | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
|--------------------------|------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| MCTP transport<br>header | 32 bits    | The MCTP transport header is part of each MCTP packet and provides<br>version and addressing information for the packet as well as flags and a<br>"Message Tag" field that, in conjunction with the source EID, is used to<br>identify packets that constitute an MCTP message. The MCTP transport<br>header fields are common fields that are always present regardless of the<br>physical medium over which MCTP is being used.<br>Note:<br>The positioning of the sub-fields of the MCTP transport header is allowed to<br>vary based on the physical medium binding.                                                                             |
| RSVD                     | 4 bits     | (Reserved) Reserved for future definition by the MCTP base specification.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Hdr version              | 4 bits     | (Header version) Identifies the format, length, physical framing, and data<br>integrity mechanism used to transfer the MCTP common fields in messages<br>on a given physical medium. For this version of MCTP, this field shall be set<br>to 0001b.                                                                                                                                                                                                                                                                                                                                                                                                  |
| Destination              | 8 bits     | The EID for the endpoint to receive the MCTP packet.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| endpoint ID              |            | A few EID values are reserved for specific routing.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
|                          |            | See Table 2 – Special endpoint IDs.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Source endpoint<br>ID    | 8 bits     | The EID of the originator of the MCTP packet. See Table 2 – Special<br>endpoint IDs.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| SOM                      | 1 bit      | (Start Of Message) Set to 1b if this packet is the first packet of a message.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| EOM                      | 1 bit      | (End Of Message) Set to 1b if this packet is the last packet of a message.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Pkt Seq #                | 2 bits     | (Packet sequence number) For messages that span multiple packets, the<br>packet sequence number increments modulo 4 on each successive packet.<br>This allows the receiver to detect up to three successive missing packets<br>between the start and end of a message. Though the packet sequence<br>number can be any value (0-3) if the SOM bit is set, it is recommended that it<br>is an increment modulo 4 from the prior packet with an EOM bit set. After the<br>SOM packet, the packet sequence number shall increment modulo 4 for each<br>subsequent packet belonging to a given message up through the packet<br>containing the EOM flag. |
| TO                       | 1 bit      | The TO (Tag Owner) bit identifies whether the message tag was originated<br>by the endpoint that is the source of the message or by the endpoint that is<br>the destination of the message. The Message Tag field is generated and<br>tracked independently for each value of the Tag Owner bit. MCTP message<br>types may overlay this bit with additional meaning, for example using it to<br>differentiate between "request" messages and "response" messages.<br>Set to 1b to indicate that the source of the message originated the message<br>tag.                                                                                             |
| Msg tag                  | 3 bits     | (Message tag) Field that, along with the Source Endpoint IDs and the Tag<br>Owner (TO) field, identifies a unique message at the MCTP transport level.<br>Whether other elements, such as portions of the MCTP Message Data field,<br>are also used for uniquely identifying instances or tracking retries of a<br>message is dependent on the message type.<br>A source endpoint is allowed to interleave packets from multiple messages to<br>the same destination endpoint concurrently, provided that each of the                                                                                                                                |
|                          |            | messages has a unique message tag.<br>When request/response message exchange is used and the Tag Owner (TO)<br>bit is set to 1 in the request, a responder should return the same Message<br>Tag with the Message Tag Owner bit cleared to 0 in the corresponding<br>response Message.<br>For messages that are split up into multiple packets, the Tag Owner (TO) and<br>Message Tag bits remain the same for all packets from the SOM through the<br>EOM.                                                                                                                                                                                          |

| Field Name             | Field Size               | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
|------------------------|--------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Message body           | See<br>description       | The message body represents the payload of an MCTP message. The<br>message body can span multiple MCTP packets.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| IC                     | 1 bit                    | (MCTP integrity check bit) Indicates whether the MCTP message is covered<br>by an overall MCTP message payload integrity check. This field is required to<br>be the most significant bit of the first byte of the message body in the first<br>packet of a message along with the message type bits.<br>0b = No MCTP message integrity check<br>1b = MCTP message integrity check is present                                                                                                                                                                                                                                                                                                                                                                                             |
| Message type           | 7 bits                   | Defines the type of payload contained in the message data portion of the<br>MCTP message. This field is required to be contained in the least-significant<br>bits of the first byte of the message body in the first packet of a message.<br>Like the fields in the MCTP transport header, the message type field is one of<br>the common MCTP fields that are present independent of the transport over<br>which MCTP is being used. Unlike the MCTP transport header, however, the<br>message type field is only required to be present in the first packet of a<br>particular MCTP message, whereas the MCTP transport header fields are<br>present in every MCTP packet. See DSP0239 and Table 3 for information on<br>message type values.                                          |
| Message header         | 0 to M bytes             | Additional header information associated with a particular message type, if<br>any. This will typically only be contained in the first packet of a message, but<br>a given message type definition can define header fields as required for any<br>packet.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Message data           | 0 to N bytes             | Data associated with the particular message type. Defined according to the<br>specifications for the message type.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| MCTP packet<br>payload | See<br>description       | The packet payload is the portion of the message body that is carried in a<br>given MCTP packet. The packet payload is limited according to the rules<br>governing packet payload and transfer unit sizes. See 8.4, Packet payload<br>and transmission unit sizes, for more information.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Msg integrity<br>check | Message<br>type-specific | (MCTP message integrity check) This field represents the optional presence<br>of a message type-specific integrity check over the contents of the message<br>body. If present, the Message integrity check field shall be carried in the last<br>bytes of the message body. The particular message type definition will<br>specify whether this is required, optional, or not to be used, the field size, and<br>what algorithm is to be used to generate the field. The MCTP base protocol<br>also does not specify whether this field is required on single packet<br>messages (potentially dependent on transmission unit size) or is only<br>required on multiple packet messages. Use of the Msg integrity check field is<br>specific to the particular message type specification. |

## <span id="page-25-0"></span>807 **8.3 Special endpoint IDs**

808 The following table lists EID values that are reserved or assigned to specific functions for MCTP.

## 809 **Table 2 – Special endpoint IDs**

<span id="page-25-1"></span>

| Value                     | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
|---------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Destination endpoint ID 0 | Null Destination EID. This value indicates that the destination EID value is to<br>be ignored and that only physical addressing is used to route the message to<br>the destination on the given bus. This enables communication with devices<br>that have not been assigned an EID. Because the physical addresses between<br>buses are not guaranteed to be unique, MCTP does not support bridging<br>messages with a null destination EID between different buses. |

| Value                    | Description                                                                                                                                                                                                                                                                                                                                                                                                              |
|--------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Source endpoint ID 0     | Null Source EID. This value indicates a message is coming from an endpoint<br>that is using physical addressing only. This would typically be used for<br>messages that are delivered from an endpoint that has not been assigned an<br>EID. Because the physical addresses between buses are not guaranteed to be<br>unique, MCTP does not support bridging messages with a null source EID<br>between different buses. |
| Endpoint IDs 1 through 7 | Reserved for future definition.                                                                                                                                                                                                                                                                                                                                                                                          |
| Endpoint ID 0xFF         | Broadcast EID. Reserved for use as a broadcast EID on a given bus. MCTP<br>network-wide broadcasts are not supported. Primarily for use by the MCTP<br>control message type.                                                                                                                                                                                                                                             |
| All other values         | Available for assignment and allocation to endpoints.                                                                                                                                                                                                                                                                                                                                                                    |

## <span id="page-26-0"></span>**8.4 Packet payload and transmission unit sizes**

## **8.4.1 Packet payload size**

 For MCTP, the size of a transmission unit is defined as the size of the packet payload that is carried in an MCTP packet.

## **8.4.2 Baseline transmission unit**

- The following are key information points regarding baseline transmission unit:
- The baseline transmission unit (minimum transmission unit) size for MCTP is 64 bytes.
- A message terminus that supports MCTP control messages shall always accept valid packets that have a transmission unit equal to or less than the baseline transmission unit. The message terminus is also allowed to support larger transmission units.
- The transmission unit of all packets in a given message shall be the same size, except for the transmission unit in the last packet (packet with EOM bit = 1b). Except for the last packet, this size shall be at least the baseline transmission unit size.
- The size of the transmission unit in the last packet shall be less than or equal to the transmission unit size used for the other packets (if any).
- If a transmission unit size larger than the baseline transmission unit is negotiated, the transmission unit of all packets shall be less than or equal to the negotiated transmission unit size. (The negotiation mechanism for larger transmission units between endpoints is message type-specific and is out of scope of this specification.)
- A given endpoint may negotiate additional restrictions on packet sizes for communication with another endpoint, as long as the requirements of this clause are met.
- All message types shall include support for being delivered using packets that have a transmission unit that is no larger than the baseline transmission unit. This is required to support bridging those messages in implementations where there are MCTP bridges that only support the baseline transmission unit.

### <span id="page-26-1"></span>**8.5 Maximum message body sizes**

 The Message Body can span multiple packets. Limitations on message body sizes are message type-specific and are documented in the specifications for each message type.

## <span id="page-27-0"></span>**8.6 Message assembly**

 The following fields (and *only* these fields) are collectively used to identify the packets that belong to a given message for the purpose of message assembly on a particular destination endpoint.

- Msg Tag (Message Tag)
- TO (Tag Owner)
- Source Endpoint ID

 As described in [3.2,](#page-11-0) together these values identify the message terminus on the destination endpoint. For a given message terminus, only one message assembly is allowed to be in process at a time.

## <span id="page-27-1"></span>**8.7 Dropped packets**

 Individual packets are dropped (silently discarded) by an endpoint or physical layer under the following conditions. These packets are discarded before being checked for acceptance or rejection for message assembly. Therefore, these packets will *not* cause a message assembly to be started or terminated.

## • **Unexpected "middle" packet or "end" packet**

 A "middle" packet (SOM flag = 0 and EOM flag = 0) or "end" packet (SOM flag = 0 and EOM flag = 1) for a multiple-packet message is received for a given message terminus without first having received a corresponding "start" packet (where the "start" packet has SOM flag = 1 and EOM flag = 0) for the message.

#### • **Bad packet data integrity or other physical layer error**

 A packet is dropped at the physical data-link layer because a data integrity check on the packet at that layer was invalid. Other possible physical layer errors may include framing errors, byte alignment errors, packet sizes that do not meet the physical layer requirements, and so on.

#### • **Bad, unexpected, or expired message tag**

 A message with TO bit = 0 was received, indicating that the destination endpoint was the originator of the tag value, but the destination endpoint did not originate that value, or is no longer expecting it. (MCTP bridges do not check message tag or TO bit values for messages that are not addressed to the bridge's EID, or to the bridge's physical address if null-source or destination-EID physical addressing is used.)

#### • **Un-routable EID**

 An MCTP bridge receives an EID that the bridge is not able to route (for example, because the bridge did not have a routing table entry for the given endpoint).

#### • **Bad header version**

The MCTP header version (Hdr Version) value is not a value that the endpoint supports.

### • **Unsupported transmission unit**

The transmission unit size is not supported by the endpoint that is receiving the packet.

 Individual packets should be dropped (silently discarded) by an endpoint under the following conditions. These packets are discarded before being checked for acceptance or rejection for message assembly. Therefore, these packets will *not* cause a message assembly to be started or terminated.

#### • **Unknown destination EID**

 A packet is received at the physical address of a device which has a valid EID, but the destination EID does not match the EID for the device.

#### • **Unsupported message type**

<span id="page-28-0"></span>An MCTP endpoint receives a message type which is not supported by that endpoint.

## **8.8 Starting message assembly**

- Multiple-packet message assembly begins when the endpoint corresponding to the destination EID in the packet receives a valid "start" packet (packet with SOM = 1b and EOM = 0b).
- A packet with both SOM = 1b and EOM = 1b is considered to be a single-packet message, and is not assembled per se.
- Both multiple- and single-packet messages are subject to being terminated or dropped based on conditions listed in the following clause.

## <span id="page-28-1"></span>**8.9 Terminating message assembly/dropped messages**

- Message assembly is terminated at the destination endpoint and messages are accepted or dropped under the following conditions:
- **Receipt of the "end" packet for the given message**
- Receiving an "end" packet (packet with EOM = 1b) for a message that is in the process of being assembled on a given message terminus will cause the message assembly to be completed (provided that the message has not been terminated for any of the reasons listed below). This is normal termination. The message is considered to be accepted at the MCTP base protocol level.
- **Receipt of a new "start" packet**
- Receiving a new "start" packet (packet with SOM = 1b) for a message to the same message terminus as a message assembly already in progress will cause the message assembly in process to be terminated. All data for the message assembly that was in progress is dropped. The newly received start packet is not dropped, but instead it begins a new message assembly. This is considered an error condition.
- **Timeout waiting for a packet**
- Too much time occurred between packets of a given multiple-packet message. All data for the message assembly that was in progress are dropped. This is considered an error condition. The timeout interval, if specified, is specific to the transport binding specification. (A binding specification may choose to not define a value for this timeout.)
- **Out-of-sequence packet sequence number**
- For packets comprising a given multiple-packet message, the packet sequence number for the most recently received packet is not a mod 4 increment of the previously received packet's sequence number. All data for the message assembly that was in progress is dropped. This is considered an error condition.

#### • **Incorrect transmission unit**

 An implementation may terminate message assembly if it receives a "middle" packet (SOM = 0b and EOM = 0b) where the MCTP packet payload size does not match the MCTP packet payload size for the start packet (SOM = 1b and EOM bit = 0b). This is considered an error condition.

#### • **Bad message integrity check**

- For single- or multiple-packet messages that use a message integrity check, a mismatch with the message integrity check value can cause the message assembly to be terminated and the entire message to be dropped, unless it is overridden by the specification for a particular message type.
- NOTE: The message integrity check is considered to be at the message-type level error condition rather than an error at the MCTP base protocol level.

## <span id="page-29-0"></span>**8.10 Dropped messages**

- An endpoint may drop a message if the message type is not supported by the endpoint. This can happen in any one of the following ways:
- The endpoint can elect to not start message assembly upon detecting the invalid message type in the first packet.
- The endpoint can elect to terminate message assembly in process.
- <span id="page-29-1"></span>• The endpoint can elect to drop the message after it has been assembled.

## **8.11 MCTP versioning and message type support**

## **8.11.1 MCTP version support**

- There are three types of versioning information that can be retrieved using MCTP control messages:
- MCTP base specification version information
- MCTP packet header version information
- Message type version information

 The version of the MCTP base specification that is supported by a given endpoint is obtained through the Get MCTP Version Support command. This command can also be used to discover whether a particular message type is supported on an endpoint, and if so, what versions of that message type are supported.

The Header Version field in MCTP packets identifies the media-specific formatting used for MCTP

packets. It can also indicate a level of current and backward compatibility with versions of the base

 specification, as specified by the header version definition in each medium-specific transport binding specification.

## **8.11.2 Compatibility with future versions of MCTP**

 An Endpoint may choose to support only certain versions of MCTP. The command structure along with the Get MCTP Version Support command allows endpoints to detect and restrict the versions of MCTP used by other communication endpoints. To support this, all endpoints on a given medium are required to implement MCTP Version 1.0.x control commands or later 1.x Version for initialization and version support discovery.

## <span id="page-30-0"></span>951 **8.12 MCTP message types**

952 [Table 3](#page-30-3) defines the values for the Message Type field for different message types transported through

953 MCTP. The MCTP control message type is specified within this document. Baseline requirements for the

954 Vendor Defined – PCI and Vendor Defined – IANA message types are also specified within this

955 document. All other message types are specified in the [DSP0239](#page-8-9) companion document to this

956 specification.

957 NOTE: A device that supports a given message type is permitted to not support that message type equally across 958 all buses that connect to the device.

## 959 **Table 3 – MCTP Message Types Used in this Specification**

<span id="page-30-3"></span>

| Message Type          | Message<br>Type Code | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
|-----------------------|----------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| MCTP control          | 0x00                 | Messages used to support initialization and configuration of MCTP<br>communication within an MCTP network. The messages and<br>functions for this message type are defined within this specification.                                                                                                                                                                                                                                                                                                                                                                                 |
| Vendor Defined – PCI  | 0x7E                 | Message type used to support VDMs where the vendor is identified<br>using a PCI-based vendor ID. The specification of the initial<br>message header bytes for this message type is provided within this<br>specification. Otherwise, the message body content is specified by<br>the vendor, company, or organization identified by the given vendor<br>ID.                                                                                                                                                                                                                           |
| Vendor Defined – IANA | 0x7F                 | Message type used to support VDMs where the vendor is identified<br>using an IANA-based vendor ID. (This format uses an "enterprise<br>number" that is assigned and maintained by the Internet Assigned<br>Numbers Authority, www.iana.org, as the means of identifying a<br>particular vendor, company, or organization.) The specification of<br>the initial message header bytes for this message type is provided<br>within this specification. Otherwise, the message body content is<br>specified by the vendor, company, or organization identified by the<br>given vendor ID. |

## <span id="page-30-1"></span>960 **8.13 Security**

 The basic premise of MCTP is that higher layer protocols will fulfill security requirements (for example, confidentiality and authentication) for communication of management data. This means that the data models carried by MCTP shall fulfill the security requirements of a given management transaction. The MCTP protocol itself will not define any additional security mechanisms.

## <span id="page-30-2"></span>965 **8.14 Limitations**

 MCTP has been optimized for communications that occur within a single computer system platform. It has not been designed to handle problems that can typically occur in a more generic inter-system networking environment. In particular, compared to networking protocols such as IP and TCP/IP, MCTP has the following limitations:

- 970 MCTP has limited logical addressing. MCTP been optimized for the small number of endpoints 971 that are expected to be utilized within the platform. The 8-bit range of EIDs is limited compared 972 to the ranges available for IP addresses.
- 973 MCTP assumes an MCTP network implementation that does not include loops. There is no 974 mechanism defined in MCTP to detect or reconcile implementations that have connections that 975 form routing loops.

- MCTP assumes a network topology where all packets belonging to a given message will be delivered through the same route (that is, MCTP does not generally support some packets for a message arriving by one route, while other packets for the message arrive by a different route).
- MCTP does not support out-of-order packets for message assembly.
- The MCTP base protocol does not address flow control or congestion control. These behaviors, if required, are specified at the physical transport binding level or at the message type or higher level.
- MCTP is not specified to handle duplicate packets at the base protocol message assembly level. If a duplicate packet is received and passed on to MCTP message assembly, it can cause the entire message assembly to be terminated.
- NOTE: Transport bindings are not precluded from including mechanisms for handling duplicate packets at the physical transport level.

## <span id="page-31-0"></span>**8.15 MCTP discovery and addressing**

## **8.15.1 Overview**

 This clause describes how MCTP endpoints, and their capabilities are discovered by one another, and how MCTP endpoints are provisioned with the addresses necessary for MCTP communication.

- MCTP discovery occurs over the course of several discrete, ordered steps:
- Bus enumeration
- Bus address assignment
- MCTP capability discovery
- Endpoint ID assignment
- Distribution and use of routing information
- This clause gives an overview of the methods used for accomplishing each of these steps in various operational scenarios. Clause [12](#page-66-0) gives details on the messages used to implement these operations.

## **8.15.2 Bus enumeration**

 This step represents existing bus enumeration. (The actions taken in this step are specific to a given medium.) Because enumeration of devices on the physical bus is medium-specific, this information is provided in the transport binding specification for the medium.

## **8.15.3 Bus address assignment**

 MCTP endpoints require a bus address that is unique to a given bus segment. This step deals with assignment of these addresses. Some bus types (such as PCIe) have built-in mechanisms to effectively deal with this. Others (such as SMBus/I2C) require some additional consideration. Because bus address assignment is medium-specific, this information is provided in the transport binding specification for the medium.

### **8.15.4 MCTP capability discovery**

- Capability discovery deals with the discovery of the characteristics of individual MCTP endpoints.
- Capabilities that can be discovered include what message types are supported by an endpoint and what
- message type versions are supported. See [8.11](#page-29-1) for a description of the methods used to accomplish
- capability discovery.

## **8.15.5 Endpoint ID assignment**

Endpoint IDs are system-wide unique IDs for identifying a specific MCTP endpoint. They can be

 dynamically assigned at system startup or hot-plug insertion. See [8.18](#page-33-0) for a description of the methods used to accomplish EID assignment.

## **8.15.6 Distribution and use of routing information**

 Bridging-capable MCTP endpoints need routing information to identify the next hop to forward a message to its final destination. See clause [9](#page-38-1) for a description of how routing information is conveyed between MCTP endpoints.

## <span id="page-32-0"></span>**8.16 Devices with multiple media interfaces**

 MCTP fully supports management controllers or managed devices that have interfaces on more than one type of bus. For example, a device could have both a PCI Express (PCIe) and an SMBus/I2C interface. In this scenario, the device will typically have a different EID for each interface. (Bridges can include instantiations that have an endpoint shared across multiple interfaces; see [9.1.3](#page-41-1) for more information.)

This concept can be useful in different operational scenarios of the managed system. For example,

 typically a PCIe interface will be used during [ACPI](#page-8-10) "S0" power states (when the system is fully powered up), which will provide significantly higher bandwidths, whereas the SMBus/I2C interface could be used

for "S3–S5" low-power sleep states.

 The baseline transmission unit is specified to be common across all media, enabling packets to be routed between different media without requiring bridges to do intermediate assembly and disassembly

operations to handle differences in packet payload sizes between different media.

Devices that support multiple media interfaces shall meet the command requirements of this specification

 and the associated transport binding specification for each enabled interface. For a given message type, the device may implement the same message type –specific commands on all MCTP interfaces,

regardless of the medium, unless otherwise specified by the message type specification.

## <span id="page-32-1"></span>**8.17 Peer transactions**

Endpoints can intercommunicate in a peer-to-peer manner using the physical addressing on a given bus.

A special value for the EID is used in cases when the physical address is known, but the EID is not

known. This capability is used primarily to support device discovery and EID assignment. A device that

does not yet have an EID assignment is not addressed using an EID. Rather, the device gets its EID

assigned using an MCTP control command, Set Endpoint ID, which uses physical addressing only.

 Similarly, depending on the transport binding, a device can also announce its presence by sending an MCTP message to a well-known physical address for the bus owner (for example, for PCIe VDM, this would be the root complex; for SMBus/I2C, the host slave address, and so on).

It is important to note that in cases where two endpoints are on the same bus, they do not need to go

through a bridge to communicate with each other. Devices use the Resolve Endpoint ID command to ask

 the bus owner what physical address should be used to route messages to a given EID. Depending on the bus implementation, the bus owner can either return the physical address of the bridge that the

message should be delivered to, or it can return the physical address of the peer on the bus.

## <span id="page-33-0"></span>**8.18 Endpoint ID assignment and endpoint ID pools**

## **8.18.1 Overview**

 MCTP EIDs are the system-wide unique IDs used by the MCTP infrastructure to address endpoints and for routing messages across multiple buses in the system. There is one EID assigned to a given physical address. Most managed devices or management controllers will connect to just a single bus and have a single EID. A non-bridge device that is connected to multiple different buses will have one EID for each bus it is attached to.

- Bus owners are MCTP devices that are responsible for issuing EIDs to devices on a bus segment. These EIDs come from a pool of EIDs maintained by the bus owner.
- With the exception of the topmost bus owner (see [8.18.2\)](#page-33-1), a given bus owner's pool of EIDs is dynamically allocated at run-time by the bus owner of the bus above it in the hierarchy. Hot-plug devices shall have their EID pools dynamically allocated.
- Once EIDs are assigned to MCTP endpoints, it is necessary for MCTP devices involved in a transaction to understand something about the route a given message will traverse. Clause [9](#page-38-1) describes how this routing information is shared among participants along a message's route.

## <span id="page-33-1"></span>**8.18.2 Topmost bus owner**

- The topmost bus owner is the ultimate source of the EID pool from which all EIDs are drawn for a given MCTP network.
- 1. This is illustrated in [Figure 5,](#page-34-0) in which the arrows are used to identify the role of bus ownership. The arrows point outward from the bus owner for the particular bus and inward to a device that is "owned" on the bus.
- In [Figure 5,](#page-34-0) device X in diagram A and bridge X in diagram B are examples of topmost bus owners.
- Diagram A shows a device that connects to a single bus and is the topmost bus owner for the overall
- MCTP network. Diagram B shows that a bridge can simultaneously be the topmost bus owner, as well as
- the bus owner for more than one bus. The different colors represent examples of different media.

<span id="page-34-0"></span>1079 **Figure 5 – Topmost bus owners**

<span id="page-34-1"></span>1081 **Figure 6 – Split bridge**

 An implementation may need to split a bus owner or bridge across two physical devices. Such an implementation shall include a mechanism (for example, a link as shown in [Figure 6\)](#page-34-1) that enables the two parts to share a common routing table, or have individual copies of the routing table that are kept synchronized. The definition of this mechanism is outside the scope of this specification.

## <span id="page-34-2"></span>1086 **8.18.3 Use of static EIDs and static EID pools**

 In general, the only device that will require a static (pre-configured default assigned non-zero value) EID assignment will be the topmost bus owner. It needs a static EID because there is no other party to assign it an EID through MCTP. Otherwise, all other devices will have their EIDs assigned to them by a bus 1090 owner.

 The same principle applies if the device functions as an MCTP bridge. If the device is the highest device in the MCTP bus hierarchy, it will require a static pool of EIDs to be assigned to it as part of the system design. Otherwise, the device will be dynamically allocated a pool of EIDs from a higher bus owner.

 An MCTP network implementation is allowed to use static EIDs for devices other than the topmost bus owner. Typically, this would only be done for very simple MCTP networks. Other key EID assignment considerations follow:

- Endpoints that support the option of being configured for one or more static EIDs shall also support being configured to be dynamically assigned EIDs.
- No mechanism is defined in the MCTP base specification for a bridge or bus owner to discover and incorporate a static EID into its routing information. Thus, a simple endpoint that is configured with a static EID shall also be used with a bus owner that is configured to support the static EIDs for the endpoint.
- All bus owners/bridges in the hierarchy, from the topmost bus owner to the endpoint, shall have their routing configurable to support static EID routing information.
- Although an endpoint that uses a static EID shall be used with a bus owner that supports static EIDs, the reverse is not true. A bus owner that uses static EIDs does not need to require that the devices on the buses it owns be configured with static EIDs.
- How the configuration of static EIDs default value occurs is outside the scope of this specification.
- No specified mechanism exists to "force" an override of a bridge's or bus owner's routing table entries for static EIDs. That is, commands such as Allocate Endpoint IDs and Routing Information Update only affect entries that are associated with dynamic EIDs.
- MCTP does not define a mechanism for keeping routing tables updated if static EIDs are used with dynamic physical addresses. That is, static EIDs are not supported for use with dynamic physical addresses.
- Bridges can have a mix of both static and dynamic EID pools. That is, the routing table can have both static and dynamic entries and can allocate from static and dynamic EID pools. Only the dynamic EID pool is given to the bridge by the bus owner using the Allocate Endpoint IDs command. There is no specification for how a static EID pool gets configured or how a bridge decides whether to give an endpoint an EID from a static or dynamically obtained EID pool. There is also no MCTP-defined mechanism to read the static EID pool setting from the bridge.
- MCTP bridges and bus owners (except the topmost bus owner) are not required to include support for static EIDs.
- MCTP does not define a mechanism for allocating EID pools that take static EID assignments into account. That is, a bridge cannot request a particular set of EIDs to be allocated to it.
- MCTP bridges/bus owners may be configurable to use only static EIDs.

## **8.18.4 Use of static physical addresses**

 In many simple topologies, it is desirable to use devices that have statically configured physical addresses. This can simplify the implementation of the device. For example, an SMBus/I2C device that is not used in a hot-plug application would not need to support the SMBus address assignment (SMBus ARP) protocol. Fixed addresses can also aid in identifying the location and use of an MCTP device in a system. For example, if a system has two otherwise identical MCTP devices, a system vendor will know that the device at address "X" is the one at the front of the motherboard, and the device at address "Y" is at the back, because that is how they assigned the addresses when the system was designed.

- Therefore, MCTP transport bindings, such as for SMBus/I2C, are allowed to support devices being at
- static physical addresses without requiring the binding to define a mechanism that enables the bus owner
- to discover MCTP devices that are using static addresses.
- In this case, the bridge or bus owner shall have a-priori knowledge of the addresses of those devices to be able to assign EIDs to those devices and to support routing services for those devices. To support this requirement, the following requirements and recommendations are given to device vendors:
- Devices that act as bus owners or bridges and are intended to support MCTP devices that use static physical addresses should provide a non-volatile configuration option that enables the system integrator to configure which device addresses are being used for devices on each bus that is owned by the bridge/bus owner.
- The mechanism by which this non-volatile configuration occurs is specific to the device vendor. In many cases, the physical address information will be kept in some type of non-volatile storage that is associated with the device and gets loaded when the device is manufactured or when the device is integrated into a system. In other cases, this information may be coded into a firmware build for the device.

## **8.18.5 Endpoint ID assignment process for bus owners/bridges**

- The bus owner/bridge shall get its own EID assignment, and a pool of EIDs, as follows. These steps only apply to bus owner/bridge devices that are not the topmost bus owner.
- Bus owners/bridges shall be pre-configured with non-volatile information that identifies which buses they own. (How this configuration is accomplished is device/vendor specific and is outside the scope of this specification.)
- The bus owner/bridge announces its presence on any buses *that it does not own* to get an EID assignment for that bus. The mechanism by which this announcement occurs is dependent on the particular physical transport binding and is defined as part of the binding specification.
- The bus owner/bridge waits until it gets its own EID assignment for one of those buses through the Set Endpoint ID command.
- The bus owner/bridge indicates the size of the EID pool it requires by returning that information in the response to the Set Endpoint ID command.
- For each bus where the bus owner/bridge is itself an "owned" device, the bus owner/bridge will be offered a pool of EIDs by being sent an Allocate Endpoint IDs command from the bus owner.
- The bus owner/bridge accepts allocations only from the bus of the "first" bus owner that gives it the allocation, as described in the Allocate Endpoint IDs command description in 8.10. If it gets allocations from other buses, they are rejected.
- The bus owner can now begin to build a routing table for each of the buses that it owns and accept routing information update information. Refer to [9](#page-38-1) for more information.

## <span id="page-36-0"></span>**8.18.6 Endpoint ID retention**

 Devices should retain their EID assignments for as long as they are in their normal operating state. Asynchronous conditions, such as device errors, unexpected power loss, power state changes, resets, firmware updates, may cause a device to require a reassignment of its EID. Devices should retain their EID assignments across conditions where they may temporarily stop responding to commands over MCTP, such as during internal resets, error conditions, or configuration updates.

## **8.18.7 Reclaiming EIDs from hot-plug devices**

 Bridges will typically have a limited pool of EIDs from which to assign and allocate to devices. (This also applies when a single bus owner supports hot-plug devices.) It is important for bridges to reclaim EIDs so

## **MCTP Base Specification DSP0236**

 that when a device is removed, the EID can later be re-assigned when a device is plugged in. Otherwise, the EID pool could become depleted as devices are successively removed and added.

EIDs for endpoints that use static addresses are not reclaimed.

No mechanism is specified in the MCTP base protocol for detecting device removal when it occurs.

Therefore, the general approach to detecting whether a device has been removed is to re-enumerate the

bus when a new device is added, and an EID or EID pool is being assigned to that device.

 The following approach can be used to detect removed hot-plug devices: The bus owner/bridge can detect a removed device or devices by validating the EIDs that are presently allocated to endpoints that are directly on the bus and identifying which EIDs are missing. It can do this by attempting to access each endpoint that the bridge has listed in its routing table as being a device that is directly on the particular bus. Attempting to access each endpoint can be accomplished by issuing the Get Endpoint ID command to the physical address of each device and comparing the returned result to the existing entry in the routing table. If there is no response to the command, or if there is a mismatch with the existing routing information, the entry should be cleared, and the corresponding EID or EID range should be returned to the "pool" for re-assignment. The bus owner/bridge can then go through the normal steps for EID assignment.

 This approach should work for all physical transport bindings, because it keeps the "removed EID" detection processing separated from the address assignment process for the bus.

 In some cases, a hot-plug endpoint may temporarily go into a state where it does not respond to MCTP control messages. Depending on the medium, it is possible that when the endpoint comes back online, it does not request a new EID assignment but instead continues using the EID it had originally assigned. If this occurs while the bus owner is validating EIDs to see if any endpoints are no longer accessible, it is possible that the bus owner will assume that the endpoint was removed and reassign its EID to a newly inserted endpoint, unless other steps are taken:

- The bus owner shall wait at least TRECLAIM seconds before reassigning a given EID (where TRECLAIM is specified in the physical transport binding specification for the medium used to access the endpoint).
- Reclaimed EIDs shall only be reassigned after all unused EIDs in the EID pool have been assigned to endpoints. Optionally, additional robustness can be achieved if the bus owner maintains a short FIFO list of reclaimed EIDs (and their associated physical addresses) and allocates the older EIDs first.
- A bus owner shall confirm that an endpoint has been removed by attempting to access it after TRECLAIM has expired. It can do this by issuing a Get Endpoint ID command to the endpoint to verify that the endpoint is still non-responsive. It is recommended that this be done at least three times, with a delay of at least 1/2 \* TRECLAIM between tries if possible. If the endpoint continues to be non-responsive, it can be assumed that it is safe to return its EID to the pool of EIDs available for assignment.

## <span id="page-37-0"></span>**8.18.8 Additional requirements for hot-plug endpoints**

 Devices that are hot-plug shall support the Get Endpoint UUID command. The purpose of this requirement is to provide a common mechanism for identifying when devices have been changed.

 Endpoints that go into states where they temporarily do not respond to MCTP control messages shall re- announce themselves and request a new EID assignment if they are "offline" for more than TRECLAIM seconds, where TRECLAIM is specified in the physical transport binding specification for the medium used to access the endpoint.

## <span id="page-38-2"></span>**8.18.9 Additional requirements for devices with multiple endpoints**

- A separate EID is utilized for each MCTP bus that a non-bridge device connects to. In many cases, it is desirable to be able to identify that the same device is accessible through multiple EIDs.
- If an endpoint has multiple physical interfaces (ports), the interfaces can be correlated to the device by using the MCTP Get Endpoint UUID command (see [12.6\)](#page-72-0) to retrieve the unique system-wide identifier.
- Devices connected to multiple buses shall support the Get Endpoint UUID command for each endpoint
- and return a common UUID value across all the endpoints. This is to enable identifying EIDs as belonging to the same physical device.

## <span id="page-38-0"></span>**8.19 Handling reassigned EIDs**

- Though unlikely, it is still possible that during the course of operation of an MCTP network, a particular EID could get reassigned from one endpoint to another. For example, this could occur if a newly hot-swap inserted endpoint device gets assigned an EID that was previously assigned to a device that was subsequently removed.
- Under this condition, it is possible that the endpoint could receive a message that was intended for the previously installed device. This is not considered an issue for MCTP control messages because the control messages are typically just used by bus owners and bridges for initializing and maintaining the MCTP network. The bus owners and bridges are aware of the EIDs they have assigned to endpoints and are thus intrinsically aware of any EID reassignment.
- Other endpoints, however, are not explicitly notified of the reassignment of EIDs. Therefore, communication that occurs directly from one endpoint to another is subject to the possibility that the EID could become assigned to a different device in the middle of communication. This shall be protected against by protocols specific to the message type being used for the communication.
- In general, the approach to protecting against this will be that other message types will require some kind of "session" to be established between the intercommunicating endpoints. By default, devices would not start up with an active session. Thus, if a new device is added and it gets a reassigned EID, it will not have an active session with the other device and the other device will detect this when it tries to communicate.
- The act of having a new EID assigned to an existing device should have the same effect. That is, if a device gets a new EID assignment, it would "close" any active sessions for other message types.
- The mechanism by which other message types would establish and track communication sessions between devices is not specified in this document. It is up to the specification of the particular message type.

## <span id="page-38-1"></span>**9 MCTP bridging**

## **9.1.1 Overview**

- One key capability provided by MCTP is its ability to route messages between multiple buses and between buses of different types. This clause describes how routing information is created, maintained, and used by MCTP bridges and MCTP endpoints. Keep the following key points in mind about MCTP bridges:
- An MCTP bridge is responsible for routing MCTP packets between at least two buses.
- An MCTP bridge is typically the bus owner for at least one of those buses.

### **MCTP Base Specification DSP0236**

## 1263 **9.1.2 Routing/bridging restrictions**

1264 [Figure 7](#page-39-0) and [Figure 8](#page-40-0) illustrate some of the supported and unsupported bridging topologies. As shown, it 1265 is acceptable for a given topology to have more than one path to get to a given EID. This can occur either

1266 because different media are used or because a redundant or failover communication path is desired in an 1267 implementation.

1268 A bridge shall not route or forward packets with a broadcast destination ID.

1269

<span id="page-39-0"></span>

## 1270 **Figure 7 – Acceptable failover/redundant communication topologies**

<span id="page-40-0"></span>1272 **Figure 8 – Routing/bridging restrictions**

## <span id="page-41-1"></span>1273 **9.1.3 EID options for MCTP bridges**

1274 An MCTP bridge that connects to multiple buses can have a single EID or multiple EIDs through which 1275 the bridge's routing configuration and endpoint functionality can be accessed through MCTP control 1276 commands. There are three general options:

- 1277 The bridge uses a single MCTP endpoint
- 1278 The bridge uses an MCTP endpoint for each bus that connects to a bus owner
- 1279 The bridge uses an MCTP endpoint for every bus to which it connects
- 1280 Examples of these different options are shown in [Figure 9,](#page-41-0) and more detailed information on the options 1281 is provided following the figure.

1282

<span id="page-41-0"></span>1283 **Figure 9 – EID options for MCTP bridges**

 A bridge has only one EID pool. To prevent issues with getting an EID pool allocation from multiple bus owners, a bridge that is accessible through multiple EIDs will only accept EID pool allocation from the first bus that allocation is received from using the Allocate Endpoint IDs command. This behavior is described in more detail in the specification of the Allocate Endpoint IDs command.

1288 If necessary, the Get Endpoint UUID command can be used to correlate that EIDs belong to the same 1289 MCTP bridge device. (This correlation is not required for normal initialization and operation of the MCTP 1290 network, but it may be useful when debugging.)

 endpoint on each bus that is connected to a bus owner (for example, P1, P2). The multiple announcements provide a level of failover capability in the EID assignment process in case a particular bus owner becomes unavailable. The multiple announcements also help support a consistent EID assignment process across bus owners. To prevent issues with getting conflicting EID assignments from multiple bus owners, the bridge will only accept EID pool allocation from the first bus that an allocation is received from using the Set Endpoint ID command. This behavior is described in more detail in the specification of the Set Endpoint ID command. The bridge shall not report the need for EID assignment on any buses that the bridge itself owns.

 A single endpoint is used to access the bridge's routing configuration and endpoint functionality. Referring to diagram (A) in [Figure 9,](#page-41-0) an implementation may elect to either have the endpoint

 functionality can be located on a "virtual bus" that is behind the routing function. In either case, the routing functionality ensures that the EID can be accessed through any of the buses to

Although there is a single endpoint, the bridge shall report the need for EID assignment for that

functionality be directly associated with a particular bus/port (for example, P1) or the

The following is a more detailed description of the different EID options for bridges:

which the bridge connects.

• **Single endpoint**

• **Endpoint for each bus connection to a bus owner**

- The bridge has one endpoint for each bus connected to a bus owner. This is shown as diagram (B) in [Figure 9.](#page-41-0) There are no explicit endpoints associated with buses that are not connected to a bus owner (for example, the buses connected to ports P3 and P4, respectively.) Because of the way packet routing works, EID A and EID B can be accessed from any of the ports connected to the bridge. Thus, the bridge's configuration functionality may be accessed through multiple EIDs. Because a separate endpoint communication terminus is associated with each port (P1, P2), the bridge can accept an EID assignment for each bus independently.
- The bridge shall only report the need for EID assignment on buses that connect to a bus owner, and only for the particular MCTP control interface that is associated with the particular bus. For example, the bridge would announce the need for EID assignment for the interface associated with EID A only through P1, and the need for EID assignment for the interface associated with EID B only through P2. The bridge shall not report the need for EID assignment on any buses that the bridge itself owns.

#### • **Endpoint for every bus connection**

- The bridge has one endpoint for each bus connected to it, as shown as diagram (C) in Figure 6. This includes buses that connect to bus owners (for example, P1, P2) and buses for which the bridge is the bus owner (for example, P3, P4). Because of the way packet routing works, any of these EIDs can be accessed from any of the ports connected to the bridge.
- Because a separate endpoint communication terminus is associated with each owned port (P1, P2), the bridge can accept an EID assignment for the bus owners of each bus independently. The EIDs associated with the buses that the bridge itself owns (for example, P3, P4) shall be taken out of the EID pool that is allocated to the bridge.
- The bridge shall only report the need for EID assignment on buses that connect to a bus owner, and only for the particular MCTP control interface that is associated with the particular bus. For example, the bridge would announce the need for EID assignment for the interface associated with EID A only through P1, and the need for EID assignment for the interface associated with EID B only through P2. The bridge shall not report the need for EID assignment on any buses that the bridge itself owns.

## **9.1.4 Routing table**

 An MCTP bridge maintains a routing table where each entry in the table associates either a single EID or a range of EIDs with a single physical address and bus ID for devices that are on buses that are directly connected to the bridge.

 If the device is a bridge, there will typically be a range of EIDs that are associated with the physical address of the bridge. There may also be an entry with a single EID for the bridge itself.

## **9.1.5 Bridging process overview**

When a bridge receives an MCTP packet, the following process occurs:

- 1) The bridge checks to see whether the destination EID in the packet matches or falls within the range of EIDs in the table.
- 1) If the EID is for the bridge itself, the bridge internally consumes the packet.
- 2) If there is a match with an entry in the routing table, the following steps happen:
- The bridge changes the physical addresses in the packet and reformats the medium-specific header and trailer fields as needed for the destination bus.
- The destination physical address from the source bus is replaced with the destination physical address for the destination bus obtained from the entry in the routing table.
- The bridge replaces the source physical address in the packet it received with the bridge's own physical address on the target bus. This is necessary to enable messages to be routed back to the originator.
- Packet-specific transport header and data integrity fields are updated as required by the particular transport binding.
- 3) If there is no match, packets with EID values that are not in the routing table are silently discarded.

## **9.1.6 Endpoint operation with bridging**

 A bridge does not track the packet transmissions between endpoints. It simply takes packets that it receives and routes them on a per-packet basis based on the destination EID in the packet. It does not pay attention to message assembly or disassembly or message type-specific semantics, such as request/response semantics, for packets that it routes to other endpoints.

 Most simple MCTP endpoints will never need to know about bridges. Typically, another endpoint will initiate communication with them. The endpoint can then simply take the physical address and source EID information from the message and use that to send messages back to the message originator.

 An endpoint that needs to originate a "connection" to another MCTP endpoint does need to know what physical address should be used for messages to be delivered to that endpoint. To get this information, it needs to query the bus owner for it. An endpoint knows the physical address of the bus owner because it saved that information when it got its EID assignment.

 The Resolve Endpoint ID command requests a bus owner to return the physical address that is to be used to route packets to a given EID. (This is essentially the MCTP equivalent of the ARP protocol that is used to translate IP addresses to physical addresses.) The address that is returned in the Resolve Endpoint ID command response will either be the actual physical address for the device implementing the endpoint, or it will be the physical address for the bridge to be used to route packets to the desired

- Because the physical address format is media-specific, the format of the physical address parameter is
- documented in the specifications for the particular media-specific physical transport binding for MCTP (for example, MCTP over SMBus/I2C, MCTP over PCIe Vendor Defined Messaging, and so on).
- If endpoint A has received a message from another endpoint B, it does not need to issue a Resolve
- Endpoint ID command. Instead, it can extract the source EID and source physical address from the
- earlier message from endpoint B, and then use that as the destination EID and destination physical
  - address for the message to Endpoint B.

## **9.1.7 Routing table entries**

 Each MCTP device that does bridging shall maintain a logical routing table. A bus owner shall also typically maintain a routing table if more than one MCTP device is connected to the bus that it owns. The routing table is required because the bus owner is also the party responsible for resolving EIDs to physical addresses.

- The internal format that a device uses for organizing the routing table is implementation dependent. From a logical point of view, each entry in a routing table will be comprised of at least three elements: An EID
- range, a bus identifier, and a bus address. This is illustrated in [Figure 10.](#page-44-0)

## <span id="page-44-0"></span>**Figure 10 – Basic routing table entry fields**

 The *EID range* specifies the set of EIDs that can be reached through a particular bus address on a given bus. Because the bus ID and bus address may correspond to a particular "port" on a bridge, it is possible that there can be multiple non-contiguous ranges (multiple routing table entries) that have the same bus ID/bus address pair route. EIDs and EID ranges can be categorized into three types: downstream, upstream, and local. "Downstream" refers to EIDs that are associated with routing table entries that are for buses that are owned by the bridge that is maintaining the routing table. "Upstream" refers to EIDs that are associated with routing table entries that route to buses that are not owned by the bridge that is maintaining the routing table.

"Local" refers to the EIDs for routing table entries for endpoints that are on buses that are directly

- connected to the bridge that is maintaining the routing table. A particular characteristic of entries for local EIDs is that the Resolve Endpoint ID command is issued from the same bus that the endpoint is on. The
- bridge/bus owner delivers the physical address for that endpoint rather than the physical address
- associated with a routing function. This facilitates allowing endpoints on the same the bus to
- communicate without having to go through an MCTP routing function.
- A routing table entry may not be "local" even if two endpoints are located on the same bus. An implementation may require that different endpoints go through the routing function to intercommunicate even if the endpoints are part of the same bus.
- The *bus ID* is an internal identifier that allows the MCTP device to identify the bus that correlates to this route. MCTP does not require particular values to be used for identifying a given physical bus connection
- on a device. However, this value will typically be a 0-based numeric value.
- EXAMPLE: A device that had three buses would typically identify them as buses "0", "1", and "2".

The *bus address* is the physical address of a specific device on the bus through which the EIDs specified

- in the *EID range* can be reached. This can either be the physical address corresponding to the
- destination endpoint, or it can be the physical address of the next bridge in the path to the device. The

format of this address is specific to the particular physical medium and is defined by the physical medium

transport binding.

## **MCTP Base Specification DSP0236**

## 1421 **9.1.8 Routing table creation**

### 1422 **9.1.8.1 Overview**

1423 This clause illustrates the types of routing information that a bridge requires, and where the information 1424 comes from. This clause also describes the steps that a bus owner shall use to convey that information 1425 for a given bus.

1426 [Figure 11](#page-45-0) helps illustrate the steps that are required to completely establish the routing information

1427 required by a bridge (bridge Y). The arrows in [Figure 11](#page-45-0) point outward from the bus owner and inward to 1428 "owned" endpoints on the bus.

1429

<span id="page-45-0"></span>

| 1430 | Figure 11 – Routing table population |
|------|--------------------------------------|
|------|--------------------------------------|

## 1431 **9.1.8.2 Routing table population example**

1432 With reference to [Figure 11,](#page-45-0) the following items describe the information that bridge Y will need for routing 1433 messages in the example topology shown:

- 1434 It needs a set of EIDs allocated to it to use for itself and to allocate to other devices (for 1435 example, EIDs 14:16). These are allocated to it by the bus owner (bridge X).
- 1436 It needs a routing table that has an entry that maps EID 16 to the physical address for device E 1437 on bus 3.
- 1438 It needs routing table entries for the local devices on bus 1, which are: bridge X (EID 11), device 1439 A (EID 12), device B (EID 13), and bridge Z (EID 17), assuming that devices A and B are to be 1440 reached by bridge Y without having to go through bridge X. This information shall be given to it 1441 by the bus owner (bridge X).

- It needs to know that EIDs 8:10 are accessed through bus owner/bridge X. Therefore, it needs a routing table entry that maps the EID range 9:10 to the physical address for bridge X on bus 1. This information shall also be given to it by the bus owner (bridge X).
- It needs to know that EIDs 17:19 are accessed through bridge Z. Therefore, it needs a routing table entry that maps the EID range 17:19 to the physical address for bridge Z on bus 1. Because the bus owner (bridge X) allocated that range of EIDs to bridge Z in the first place, this information is also given to bridge Y by the bus owner (bridge X).

## <span id="page-46-0"></span>**9.1.8.3 Bus initialization example**

- Starting with the description of what bridge Y requires, the following task list shows the steps that bridge X shall take to provide routing information for bus 1. Bridge X shall:
- 1) Assign EIDs to devices A, B, C, D, bridge Y, and bridge Z. This is done using the Set Endpoint ID command. The response of the Set Endpoint ID command also indicates whether a device wants an additional pool of EIDs.
- 2) Allocate EID pools to bridge Y and bridge Z. This is done using the Allocate Endpoint IDs command.
- 3) Tell bridge Y the physical addresses and EIDs for devices A and B, bridge X (itself), and bridge Z on bus 1. This is done using the Routing Information Update command.
- 4) Tell bridge Y that EIDs 18:19 are accessed through the physical address for bridge Z on bus 1. This is also done using the Routing Information Update command. (Steps 3 and 4 can be combined and covered with one instance of the command.)
- 5) Tell bridge Z the physical addresses and EIDs for devices A and B, bridge X (itself), and bridge Y on bus 1. This is also done using the Routing Information Update command.
- 6) Tell bridge Z that EIDs 15:16 are accessed through the physical address for bridge Y on bus 1. This is also done using the Routing Information Update command. (Steps 5 and 6 can be combined and covered with one instance of the command.)
- 7) Tell bridge Y and bridge Z that EIDs 8:10 are accessed through bridge X on bus 1. This is also done using the Routing Information Update command. This step could also be combined with steps 3 and 4 for bridge Y and steps 5 and 6 for bridge Z.

## **9.1.9 Routing table updates responsibility for bus owners**

- After it is initialized for all bridges, routing table information does not typically require updating during
- operation. However, updating may be required if a bridge is added as a hot-plug device. In this case,
- when the bridge is added to the system, it will trigger the need for the bus owner to assign it an EID,
- which will subsequently cause the request for EID pool allocations, and so on. At this time, the bus owner can simply elect to re-run the steps for bus initialization as described in [9.1.8.3.](#page-46-0)

## **9.1.10 Consolidating routing table entries**

- MCTP requires that when an EID pool is allocated to a device, the range of EIDs is contiguous and
- follows the EID for the bridge itself. Thus, a bridge can elect to consolidate routing table information into
- one entry when it recognizes that it has received an EID or EID range that is contiguous with an existing
- entry for the same physical address and bus. (The reason that EID allocation and routing information
- updates are not done as one range using the same command is because of the possibility that a device
- may have already received an allocation from a different bus owner.)

## <span id="page-47-0"></span>1482 **9.2 Bridge and routing table examples**

## 1483 **9.2.1 Overview**

1484 The following examples illustrate different bridge and MCTP network configurations and the 1485 corresponding information that shall be retained by the bridge for MCTP packet routing and to support 1486 commands such as Resolve Endpoint ID and Query Hop.

 The following clauses (including [Table 4](#page-48-0) through [Table 6\)](#page-50-1) illustrate possible topologies and ways to organize the information that the bridge retains. Implementations may elect to organize and store the same information in different ways. The important aspect of the examples is to show what information is kept for each EID, to show what actions cause an entry to be created, and to show how an EID or EID range can in some cases map to more than one physical address.

1492 The examples show a possible time order in which the entries of the table are created. Note that a given 1493 implementation of the same example topology could have the entries populated in a different order. For 1494 example, if there are two bus owners connected to a bridge, there is no fixed order that the bus owners

1495 would be required to initialize a downstream bridge. Additionally, there is no requirement that bus owners

1496 perform EID assignment or EID pool allocation in a particular order. One implementation may elect to

1497 allocate EID pools to individual bridges right after it has assigned the bridge its EID. Another 1498 implementation may elect to assign all the EIDs to devices first, and then allocate the EID pools to 1499 bridges.

## 1500 **9.2.2 Example 1: Bridge D2 with an EID per "Owned" port**

1501 [Figure 12](#page-47-1) shows the routing table in a bridge (D2), where D2 has an EID associated with each bus 1502 connected to a bus owner. In this example, D1 is not implementing any internal bridging between its P1 1503 and P2. Consequently, EID9 cannot be reached by bridging through EID8 and vice versa (see [Table 4\)](#page-48-0).

1504 NOTE: If there *was* internal bridging, D1 would need to provide routing information that indicated that EID9 was 1505 reachable by going through EID8 and vice versa. In this case, D1 would provide routing information that EID range 1506 (EID8…EID9) would be accessed through D1P1a1 on SMBus and D1P2a1 on PCIe.

#### 1507 **Key: D = device, P = port, a = physical address**

1508

<span id="page-47-1"></span>1509 **Figure 12 – Example 1 Routing topology**

<span id="page-48-0"></span>

| Time | EID    | EID<br>Access<br>Port | Medium<br>Type | Access<br>Physical<br>Address | Device/Entry<br>Type                                                                                                                          | Entry Was Created and Populated<br>By                                                                                             |
|------|--------|-----------------------|----------------|-------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------|
|      | EID 10 | P1                    | SMBus          | D1P1a2                        | Bridge, Self                                                                                                                                  | Self when EID was assigned by D1                                                                                                  |
|      | EID 11 | P2                    | PCIe           | D1P2a2                        | Bridge, Self                                                                                                                                  | Self when EID was assigned by D1                                                                                                  |
|      | EID 12 | P3                    | SMBus          | D2P3a2                        | Endpoint<br>Self after D1 assigned EID pool<br>(typically the entry will not be<br>created until after the bridge D2<br>assigns EID 12 to D3) |                                                                                                                                   |
|      | EID 13 | P3                    | SMBus          | D2P3a3                        | Endpoint                                                                                                                                      | Self after D1 assigned EID pool<br>(typically the entry will not be<br>created until after the bridge D2<br>assigns EID 13 to D4) |
|      | EID 14 | P4                    | USB            | D2P4a2                        | Endpoint                                                                                                                                      | Self after D1 assigned EID pool<br>(typically the entry will not be<br>created until after the bridge D2<br>assigns EID 14 to D5) |
|      | EID 8  | P1                    | SMBus          | D1P1a1                        | Bridge                                                                                                                                        | D1 through Routing Information<br>Update command                                                                                  |
|      | EID 9  | P2                    | PCIe           | D1P2a1                        | Bridge                                                                                                                                        | D1 through Routing Information<br>Update command                                                                                  |

1510 **Table 4 – Example 1 Routing table for D2**

## 1511 **9.2.3 Example 2: Topmost bus owner D1**

1512 [Figure 13](#page-49-0) assumes the following conditions:

- 1513 D1 assigns its internal EIDs first.
- 1514 The buses are handled in the order D1P1, D1P2, D1P3.
- 1515 D1 allocates the EID pool to bridges right after it has assigned the EID to the device.

1516 Similar to Example 1, this example assumes that there is no internal bridging within D1 between P1, P2, 1517 and P3. This scenario is reflected in [Table 5.](#page-49-1)

### **MCTP Base Specification DSP0236**

#### 1518 **Key: D = device, P = port, a = physical address**

<span id="page-49-0"></span>

## <span id="page-49-1"></span>1522 **9.2.4 Example 3: Bridge D2 with single EID**

1523 [Figure 14](#page-50-0) assumes that bridge D2 has a single EID and gets its EID assignment and EID allocation

1524 through bus D1P1 first, and that bus D1P2 later gets initialized. This scenario is reflected in [Table 6.](#page-50-1)

#### 1525 **Key: D = device, P = port, a = physical address**

1526

#### <span id="page-50-0"></span>1527 **Figure 14 – Example 3 Routing topology**

### 1528 **Table 6 – Example 3 Routing table for D2**

<span id="page-50-1"></span>

| Target<br>EID | Target<br>Endpoint<br>Access<br>Port | Target EID<br>Access<br>Physical<br>Address | Device/Entry<br>Type | Entry Was Created and Populated By                                                                                              |
|---------------|--------------------------------------|---------------------------------------------|----------------------|---------------------------------------------------------------------------------------------------------------------------------|
| EID 10        | P1                                   | D1P1a2                                      | Bridge, self         | All four entries created by self (bridge) upon                                                                                  |
| EID 10        | P2                                   | D1P2a2                                      | Bridge, self         | receiving initial EID assignment from D1 through P1                                                                             |
| EID 10        | P3                                   | D2P3a1                                      | Bridge, self         |                                                                                                                                 |
| EID 10        | P4                                   | D2P4a1                                      | Bridge, self         |                                                                                                                                 |
| EID 11        | P3                                   | D2P3a2                                      | Endpoint             | Self after D1 allocated EID pool<br>(typically the entry will not be created until after the<br>bridge D2 assigns EID 11 to D3) |
| EID 12        | P3                                   | D2P3a3                                      | Endpoint             | Self after D1 allocated EID pool<br>(typically the entry will not be created until after the<br>bridge D2 assigns EID 12 to D4) |
| EID 13        | P3                                   | D2P4a2                                      | Endpoint             | Self after D1 allocated EID pool<br>(typically the entry will not be created until after the<br>bridge D2 assigns EID 13 to D5) |
| EID 8:9       | P1                                   | D1P1a1                                      | Bridge               | D1 through Routing Information Update command                                                                                   |
| EID 8:9       | P2                                   | D1P2a1                                      | Bridge               | D1 through Routing Information Update command                                                                                   |

## **MCTP Base Specification DSP0236**

## 1529 **9.2.5 Additional information tracked by bridges**

1530 In addition to the information required to route messages between different ports, a bridge has to track 1531 information to handle MCTP control commands related to the configuration and operation of bridging 1532 (shown in [Table 7\)](#page-51-1).

## 1533 **Table 7 – Additional information tracked by bridges**

<span id="page-51-1"></span>

| What                                                                                                                  | Why                                                                                                                                                                                                                                                                                                                                                           |
|-----------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Which buses are connected to a bus owner                                                                              | This information tells the bridge from which buses it<br>should request EID assignment. This will typically be<br>accomplished as a non-volatile configuration or<br>hardware-strapping option for the bridge.                                                                                                                                                |
| Which bus the bridge received its EID assignment<br>through the Set Endpoint ID command                               | If the bridge uses a single EID that is shared across<br>multiple "owned" buses, this information is used to<br>track which bus the request came in on, so that the<br>bridge can reject EID assignment requests from<br>other buses.                                                                                                                         |
| Which bus it received the Routing Information<br>Update command from for creating a particular<br>routing table entry | This information is required so that if a future Routing<br>Information Update command is received, the bridge<br>will update only the entries corresponding to that<br>bus.                                                                                                                                                                                  |
| Which bus it received its EID pool allocation from<br>through the Allocate Endpoint IDs command                       | This information is used to track which bus the<br>request came in on so that the bridge can reject EID<br>pool allocations from other buses.                                                                                                                                                                                                                 |
| The physical medium and physical addressing<br>format used for each port                                              | This information is used to provide the correctly<br>formatted response to commands such as Resolve<br>Endpoint ID and for bridging MCTP packets between<br>the different buses that the bridge supports. Because<br>this is related to the physical ports and hardware of<br>the bridge, this information will typically be "hard<br>coded" into the bridge. |

## <span id="page-51-0"></span>1534 **9.3 Endpoint ID resolution**

## 1535 **9.3.1 General**

1536 When a device uses the Resolve Endpoint ID command to request the resolution of a given endpoint to a 1537 physical address, the bridge shall respond based on which bus the request came in on.

1538 For example, consider [Figure 15.](#page-52-0) If device A wishes to get the physical address needed to send a

1539 message to device C, it sends a Resolve Endpoint ID command to bus owner bridge X through address 1540 Ax1. Because device A shall go through bridge X to get to device C, bridge X responds with its physical

1541 address Ax1.

1542 When device B wishes to know the address to use to communicate with device C, it sends a Resolve

- 1543 Endpoint ID request to bridge X through address Ax2. In this case, bridge X can respond by giving device 1544 B the direct physical address of device C on bus 2, Ac2.
- 1545 Thus, the Resolve Endpoint ID command can return a different response based on the bus from which 1546 the Resolve Endpoint ID command was received.

notation:

Ab2 = physical Address of device b on bus 2.

### 1547

## <span id="page-52-0"></span>1548 **Figure 15 – Endpoint ID resolution**

## 1549 **9.3.2 Resolving multiple paths**

1550 Cases can occur where there can be more than one possible path to a given EID. A likely scenario is 1551 shown in [Figure 16.](#page-53-1) In [Figure 16,](#page-53-1) assume that the system topology supports cards that connect to either 1552 SMBus, PCIe, or both. Bridge X is the bus owner for both buses.

1553 NOTE: This is a logical representation of MCTP buses. Physically, the buses may be formed of multiple physical 1554 segments, as would be the case if one of the MCTP buses was built using PCIe.

1555 As shown, card C contains a bridge that connects to both buses. Thus, the device with EID 100 can be 1556 reached either from bus 1 or bus 2.

1557 If device D wishes to send a message to EID 100, bridge X can choose to route that message either

1558 through bus 1 or bus 2. MCTP does not have a requirement on how this is accomplished. The general

1559 recommendation is that the bridge preferentially selects the faster available medium. In this example, that 1560 would be PCIe.

1561 NOTE There are possible topologies where that simple rule is permitted to not yield the preferred path to a device.

1562 However, in most common implementations in PC systems, this approach should be effective. A vendor making a

1563 bridge device may consider providing configuration options to enable alternative policies.

<span id="page-53-1"></span>

| 1565 | Figure 16 – Resolving multiple paths |
|------|--------------------------------------|
|      |                                      |

## <span id="page-53-0"></span>**9.4 Bridge and bus owner implementation recommendations**

## **9.4.1 Overview**

 This clause provides recommendations on EID pool and routing table sizes for devices that implement bridge and bus owner functionality.

## **9.4.2 Endpoint ID pool recommendations**

 The system design should seek to minimize the number of devices that need to allocate EID pools to hot- plug devices or add-in cards. If feasible, the system design should have all busses that support hot-plug devices/add-in cards owned by a single device.

 If only one device handles the hot-plug devices and add-in cards, it will be simpler for the system integrator to configure devices and allocate EID pools. Because any other bridges in the system that do not handle hot-plug devices only need to handle a fixed number of MCTP devices, it will be known at design time how large an EID pool will be required. The remaining number of EIDs can then simply be allocated to the single device that handles the hot-plug devices and add-in cards.

 To support this, it is recommended that devices that operate as bridges include a non-volatile configuration option that enables the system integrator to configure the size of the EID pool they request.

## **9.4.3 Routing table size recommendations**

 This clause provides some initial recommendations and approaches on how to determine what target routing table entry support to provide in a device.

#### • **PCIe slots**

- To provide entries to support devices that plug into PCIe slots, assume that each slot may support both PCIe and SMBus endpoints and provide support for at least two endpoints per bus type.
- This means providing support for at least four directly connected endpoints per card. (Other endpoints may be behind bridges on the card, but this does not affect the routing table size for the bus owner.) This implies at least four routing table entries per PCIe slot. Thus, a device that

 was designed to support system implementations with eight PCIe slots should have support for 32 routing table entries.

#### • **Planar PCIe devices**

 In most PC systems, PCIe would be typically implemented as a single MCTP bus owned by a single device as the bus owner. Thus, the number of static devices should be proportional to the number of PCIe devices that are built into the motherboard.

 Typically, this is fewer than eight devices. Thus, it is recommended to support at least eight entries for static PCIe devices.

#### • **Static SMBus/I2C MCTP devices**

 The routing table should also be sized to support an additional number of "static" devices on owned buses. At this time, it is considered unlikely that more than a few MCTP devices would be used on a given SMBus/I2C bus. Most devices would be non-intelligent sensor and I/O devices instead. Conservatively, it is recommended that at least four entries be provided for each SMBus/I2C bus that the device owns.

#### Example 1: "client" capable device

| 1606 | Four PCIe slots                  |  | 16 routing table entries |
|------|----------------------------------|---|--------------------------|
| 1607 | Two owned SMBus/I2C busses       |  | +8 entries               |
| 1608 | Static PCIe device support       |  | +8 entries               |
| 1609 |                                  |   | ~32 entries or more      |
| 1610 | Example 2: volume server capable |   |                          |
| 1611 | Eight PCIe slots                 |  | 32 routing table entries |
| 1612 | Four owned SMBus/I2C busses      |  | +16 entries              |
| 1613 | Static PCIe device support       |  | +8 entries               |
| 1614 |                                  |   | ~56 entries or more      |

## <span id="page-54-0"></span>**9.5 Path and transmission unit discovery**

## **9.5.1 Overview**

 The transmission unit is defined as the size of the MCTP packet payload that is supported for use in MCTP message assembly for a given message. The supported transmission unit sizes are allowed to vary on a per-message type basis.

Intermediate bridges and physical media can limit the transmission unit sizes between endpoints.

- Therefore, the MCTP control protocol specifies a mechanism for discovering the transmission unit support for the path between endpoints when one or more bridges exist in the path between the endpoints.
- The mechanism for path transmission unit discovery also enables the discovery of the bridges and number of "hops" that are used to route an MCTP packet from one endpoint to another.

## **9.5.2 Path transmission unit negotiation**

The MCTP control protocol only specifies how to discover what the path transmission unit size is for the

path between endpoints. The MCTP control protocol does not specify a generic mechanism for

discovering what transmission unit sizes a particular endpoint supports for a given message type.

Discovery and negotiation of transmission unit sizes for endpoints, if supported, is specified by the

definition of the particular message type.

### **MCTP Base Specification DSP0236**

### 1631 **9.5.3 Path transmission unit discovery process overview**

1632 This clause describes the process used for path transmission unit discovery. The discovery process

1633 described here is designed to enable one endpoint to discover the path and transmission unit support for 1634 accessing a particular "target" endpoint. It does not define a general mechanism for enabling an endpoint

1635 to discover the path between any two arbitrary endpoints. For example, referring to

1636 [Figure 17,](#page-55-0) the process defines a way for the endpoint at EID 9 to discover the path/transmission unit

1637 support on the route to endpoint at EID 14, but this process does not define a process for EID 9 to

1638 discover the path/transmission unit support between EID 11 and EID 14.

1639

<span id="page-55-0"></span>

## 1640 **Figure 17 – Example path routing topology**

1641 The following example provides an overview of the path/transmission unit discovery process. The

1642 example presumes that the MCTP network has already been initialized. Referring to

1643 [Figure 17,](#page-55-0) the endpoint with EID 9 wishes to discover the path used to access the endpoint with EID 14.

1644 This discovery is accomplished using just two commands, Resolve Endpoint ID and Query Hop, as 1645 follows:

- 1646 1) EID 9 first issues a Resolve Endpoint ID command to the bus owner, EID 8, with EID 14 as the EID 1647 to resolve.
- 1648 2) EID 8 returns the physical address and EID of the bridge, EID 10 in the Resolve Endpoint ID 1649 command response.
- 1650 3) EID 9 queries the bridge, EID 10, using a Query Hop command with EID 14 (the "target" EID) as the 1651 request parameter. Note that EID 9 does not need to do another Resolve Endpoint ID command 1652 because it already received the physical address of EID 10 from the original Resolve Endpoint ID 1653 command.
- 1654 4) Bridge EID 10 responds to the Query Hop command by returning EID 12, which is the EID of the 1655 next bridge required to access EID 14. The bridge EID 10 also returns the transmission unit support 1656 that it offers for routing to the target EID.
- 1657 5) EID 9 then sends a Query Hop command to the bridge at EID 12. Note that EID 9 does not need to 1658 do another Resolve Endpoint ID command because it already received the physical address of EID 1659 12 from the original Resolve Endpoint ID command.

1660 6) Bridge EID 12 responds to the Query Hop command by returning EID 14, which, because it is the 1661 EID of the target endpoint, tells EID 9 that bridge EID 12 was the last "hop" in the path to EID 6. The 1662 bridge EID 12 also returns the transmission unit support that it offers for routing to the target EID.

 7) At this point, the bridges in the path to EID 14 have subsequently been discovered and their respective transmission unit support returned. The effective transmission unit support for the path to EID 14 will be the lesser of the transmission unit support values returned by the two bridges.

## **9.5.4 Path transmission unit discovery process flowchart**

 The following flowchart (Figure 15) shows a generic algorithm for discovering the bridges in the path from one endpoint to a given target endpoint and the path transmission unit support. The flowchart has been intentionally simplified. Note that while the Query Hop command actually supports returning separate transmission unit sizes for the transmit and receive paths, the flowchart is simplified for illustration purposes and just refers to a single transmission unit for both transmit and receive.

 Additionally, [Figure 18](#page-57-1) does not show any explicit steps for error handling nor the process of handling command retries. In general, errors are most likely due to either an invalid EID being sent to the bridge (perhaps due to a programming error at the requester) or the EID not being present in the bridge's routing table. The latter condition could occur under normal operation if the requester did not realize that a routing table update had occurred because of a hot-plug update, for example. This error condition would

be indicated by the bridge responding with an ERROR\_INVALID\_DATA completion code.

#### <span id="page-57-1"></span>1679 **Figure 18 – Path transmission unit discovery flowchart**

## <span id="page-57-0"></span>1680 **9.6 Path transmission unit requirements for bridges**

 An MCTP bridge routes packets between different buses, but it does not typically interpret the packet payload contents nor does it do assembly of those packets. Exceptions to this are when the bridge is handling packets addressed to its own EID, receives a Broadcast EID, and if the bridge supports different transmission units based on message type. See [Table 32](#page-87-1) for more information.

## <span id="page-58-0"></span>**10 Rate Limiting**

## **10.1.1 Methods**

 Some MCTP bindings provide a significant transfer rate that may not be sustainable by the MCTP message receiver. It is not always possible to use the native flow control mechanisms of the medium, since they may be shared with other traffic. In order to help address this problem, Endpoints may support the following specified MCTP Rate Limiting method.

 Note: The PCIe binding is a typical example of this issue. PCIe provides significantly more bandwidth than most MCTP endpoints can consume. PCIe credits cannot be used to throttle the MCTP traffic, since this would throttle all PCIe traffic (MCTP and non-MCTP) to the device. Thus, an alternative Rate Limiting mechanism is needed. Rate limiting is performed independently in each direction and is not required to be symmetric. Rate limiting can be set for one-direction only, for both directions or not be set at all.

- The MCTP rate limit mechanism allows an endpoint on a specific medium to:
- Publish its input processing rate and whether it can rate limit its output
- Request its partner to rate limit its MCTP output traffic.

 Rate Limiting is negotiated between two endpoints and is configured on a per-EID basis such that devices having multiple EIDs should separately negotiate their Rate Limiting for each EID which supports Rate Limiting. If there are any MCTP Bridges in the path between the endpoints, the negotiated rate limit between the endpoints may not take bridge performance into account. The negotiation should take the speeds of the media for the path between the endpoints into account. Rate limiting is not specified for the bridging functionality within an MCTP Bridge (the functionality that routes MCTP packets between different ports on the bridge).

[Figure 19](#page-59-0) presents an example of message exchanges for rate limiting. In this example, the management controller (MC) wants the managed devices (MD 1, MD 2) to send data at a limited rate. The MC first queries the MDs for their rate limiting capabilities using the Query Rate Limit command. Based on those capabilities, the MC requests the maximal transmit rate configuration for the MDs using the [Request TX](#page-89-0)  [rate limit](#page-89-0) command. Conversely, the MDs may want to limit the rate that they receive data from the MC. In this case, it's the MDs that query the MC using the Query Rate Limit command, and, based on the response from the MC, requests configuration for the transmit rate from the MC using the [Request TX](#page-89-0)  [rate limit](#page-89-0) command.

 Note that the figure does not show conditions such as handling the situation where one or more of the endpoints does not support Rate Limiting, nor does it show any algorithms that the endpoints may use to determine the best end-to-end value for Rate Limiting. Devices that negotiate Rate Limiting may wish to include algorithms or tests that would indicate there are intermediate devices in the path, such as Bridges, that would require transmit rates to be set to values that are lower than just what the receiving device needs. For example, the receiving device may detect that additional Rate Limiting is needed by noticing that there are packets missing in a multi-packet MCTP message transfer sequence.

1722

<span id="page-59-0"></span>1723 **Figure 19 – Example rate limiting message exchanges**

## **10.1.2 Restrictions on rate limiting**

Message-based flow control may not utilize rate limiting. When rate limiting is active on a device which

- sends non-requested messages, then request/responses may also be affected by the rate limiting. Rate-
- limiting capable device may use rate limiting only to non-requested messages or to all messages. The transmit rate limiting operation-mode capability is reported by the device through "Transmit Rate Limiting
- operation capability" bit in Query [rate limit](#page-88-0) command response.
- The use of rate limiting shall not supersede the timing requirements that are called out in other specifications, such as the transport binding specifications. Rate limiting shall include configuration
- options that allow meeting timing requirements under nominal operating conditions.

## **10.1.3 Rate definition**

 Let B be the Maximum supported burst size and R be the Maximum output rate limit in Packets Per Second (PPS), then the traffic shall be throttled such that in *any* time window W = B/R (where B ≥

1) there are no more than B packets.

## **10.1.4 Output rate limiting capabilities parameters**

 A transmitter that supports rate limiting shall expose its rate limiting capabilities using the Query Rate Limit command. For the definition of rate limiting, a baseline-transmission packet includes the baseline transmission unit as well as any medium-specific header/trailer and MCTP transport header. This includes:

- **Maximum output rate limit:** The maximum rate in baseline transmission unit Packets/sec that the transmitting endpoint can be limited to when sending data to another endpoint.
- **Minimum output rate limit:** The minimum rate in baseline transmission unit Packets/sec that the transmitting endpoint can be limited to when sending data to another endpoint. This value is also used to define the granularity of the configurable rate limit values.
- **Maximum supported burst size**: The maximum number of consecutive baseline transmission unit Packets that the transmitter endpoint can send with minimal delay between MCTP packets.

## **10.1.5 Input processing capabilities parameters**

 A receiver can expose its input processing capabilities using the Query Rate Limit command. These parameters are informative only and should not be used to set the rate limiter of the partner. These parameters are intended to be used for visibility on the transmitter side, for performance analysis and monitoring purpose.

- The parameters exposed are:
- **Maximum allowed receive data rate:** The maximum processing rate in baseline transmission unit packets/sec that the receiving endpoint can typically process incoming traffic. The data rate is measured using a time window. This rate is defined regardless of the content being received. Thus, devices which are limited in message processing shall report the maximum allowed receive data rate for minimal-size packets.
- **Buffer Size:** this parameter defines the receive **buffer size** in bytes of the receiving endpoint.

## <span id="page-61-1"></span>**10.1.6 Defining and updating configuration parameters**

## **10.1.6.1 Rate limiting configuration parameters**

 Rate limiting requirements are defined explicitly for each endpoint by means of two parameters, the maximum allowed data rate and the maximum continuous burst size. These are defined as follows:

- **Maximum continuous burst size**: The maximum continuous burst size is defined in MCTP packets. Typically, this parameter reflects the receive buffer resources of the receiving endpoint.
- **Maximum allowed data rate**: The maximum allowed data rate is defined in baseline transmission unit packets/sec. Typically, this defines the rate at which a receiving endpoint can process incoming messages. The data rate is measured using a time window as defined above. This rate is defined regardless of the content being received. Thus, devices which are limited in message processing shall request the maximum allowed transmit data rate with Burst Size of 1 packet.
- If a device contains more than one MCTP endpoint (for example, a device that has an endpoint on
- SMBus/I2C and one on PCIe VDM) and supports setting rate limiting on these endpoints, then each rate- limiting configuration shall be independent and separately configurable. A device may include rate limiting capability for part or all of the endpoints.
- These parameters are used both by the receiver to request a specific traffic rate from the transmitter device and by the transmitter device to report the current rate-limiting values.
- When different settings are requested from different receiving endpoints, a transmitting endpoint that implements a single rate limiter shall use the smallest continuous burst size and the lowest data-rate that has been requested across the set of receiving endpoints. In a case of a single rate limiter, when traffic to multiple EIDs is active at the same time, the effective data rate to each of the receiving EIDs may be lower than the configured rate, as the aggregated data rates to all receiving EIDs will be the configured Rate Limiting settings.
- When a system is designed with devices supporting rate-limiting and devices which do not support rate-
- limiting, any device which supports rate limiting shall set its rate limiter to the negotiated rate-limiting
- settings. It is recommended that devices which do not support rate limiting are configured such that they
- will not cause buffer-overflow or data-processing rate overflow to their connected receiving endpoint. The implementation method of such a system is outside the scope of this specification.

## **10.1.6.2 Updating rate-limiting parameters**

 If an endpoint device needs to update the rate limiting settings of the other endpoint devices which are communicating with it and which are configured with rate limiting, it shall request the new settings in the sending devices using [Request TX rate limit](#page-89-0) command. Once the response to [Request TX rate limit](#page-89-0) command is received, the new rate limit is set according to the settings provided in the response. When the rate limiting settings is changed by an endpoint, the transmitting endpoint should notify the other receiving endpoint, sharing the same rate limiter, about the update using the [Update rate limit](#page-90-0) command.

# <span id="page-61-0"></span>**11 MCTP control protocol**

## **11.1.1 Overview**

 MCTP control messages are used for the setup and initialization of MCTP communications within an MCTP network. This clause defines the protocol and formatting used for MCTP control messages over MCTP.

## <span id="page-62-0"></span>1802 **11.2 Terminology**

- 1803 The terms shown in [Table 8](#page-62-2) are used when describing the MCTP control protocol.
- 1804 **Table 8 – MCTP control protocol terminology**

<span id="page-62-2"></span>

| Term                         | Description                                                                                                                                                                                       |
|------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Requester                    | The term "requester" is used to refer to the endpoint that originates an MCTP<br>control Request message.                                                                                         |
| Responder                    | The term "responder" is used to refer to the endpoint that originates an MCTP<br>control response message (that is, an endpoint that returns the response to an<br>MCTP control Request message). |
| Originator or Source         | The term "originator" or "source" is used to refer to the endpoint that originates<br>any MCTP control message: Request, Response, or Datagram.                                                   |
| Target or Destination        | The term "target" or "destination" is used to refer to the endpoint that is the<br>intended recipient of any MCTP control message: Request, Response, or<br>Datagram.                             |
| Asynchronous<br>Notification | The term "asynchronous notification" is used to refer to the condition when an<br>MCTP endpoint issues an un-requested Datagram to another MCTP endpoint.                                         |
| Broadcast                    | The term "broadcast" is used when an MCTP control Datagram is sent out onto<br>the bus using the broadcast EID.                                                                                   |

## <span id="page-62-1"></span>1805 **11.3 Control message classes**

### 1806 **11.3.1 General**

- 1807 The different types of messages shown in [Table 9](#page-62-3) are used under the MCTP control message type.

#### 1808 **Table 9 – MCTP control message types**

<span id="page-62-3"></span>

| Type              | Description                                                                                                                                                                                                                                                                                                                                              |
|-------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request           | This class of control message requests that an endpoint performs a specific<br>MCTP control operation. All MCTP control Request messages are<br>acknowledged with a corresponding Response message. (Within this<br>specification, the term "command" and "request" are used interchangeably as<br>shorthand to refer to MCTP control Request messages.) |
| Response          | This class of MCTP control message is sent in response to an MCTP control<br>Request message. The message includes a "Completion Code" field that<br>indicates whether the response completed normally. The response can also<br>return additional data dependent on the particular MCTP control Request that<br>was issued.                             |
|                   | An MCTP control Response message shall use the destination EID and<br>physical address that were used as the source EID and source physical<br>address of the corresponding MCTP control Request message.                                                                                                                                                |
| Datagram          | Datagrams are "unacknowledged" messages (that is, Datagrams do not have<br>corresponding Response messages). This class of MCTP control message is<br>used to transfer messages when an MCTP control Response message is<br>neither required nor desirable.                                                                                              |
| Broadcast Request | A broadcast message is a special type of Request that is targeted to all<br>endpoints on a given bus. All endpoints that receive the message are expected<br>to interpret the Request.                                                                                                                                                                   |

| Type               | Description                                                                                                                                                                                   |
|--------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Broadcast Datagram | A Datagram that is broadcast to all endpoints on the bus. Broadcast Datagrams<br>are "unacknowledged" messages (that is, broadcast Datagrams do not have<br>corresponding Response messages). |

## <span id="page-63-0"></span>1809 **11.4 MCTP control message format**

## 1810 **11.4.1 Overview**

1811 MCTP control messages use the MCTP control message type (see [Table 3\)](#page-30-3). Any message sent with this

1812 message type will correspond to the definitions set forth in this clause. The basic format of an MCTP 1813 control message is shown in [Figure](#page-63-2) 20. Note that the byte offsets shown in [Figure](#page-63-2) 20 are relative to the

1814 start of the MCTP message body rather than the start of the physical packet.

## 1815 **11.4.2 Use of Message Integrity Check**

1816 MCTP control messages do not use a Message Integrity Check field. Therefore, the IC bit in MCTP

1817 control messages shall always be 0b.

1818

## <span id="page-63-2"></span>1819 **Figure 20 – MCTP control message format**

## <span id="page-63-1"></span>1820 **11.5 MCTP control message fields**

1821 [Table 10](#page-63-3) lists the common fields for MCTP control messages.

#### 1822 **Table 10 – MCTP control message fields**

<span id="page-63-3"></span>

| Field Name    | Description                                                                                                                                                                                                             |
|---------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| IC*           | Message Integrity Check bit = 0b. MCTP control messages do not include<br>an overall Message Integrity check field.                                                                                                     |
| Message Type* | MCTP control = 0x00 (000_0000b). This field identifies the MCTP<br>message as being an MCTP control message.                                                                                                            |
| Rq bit        | Request bit. This bit is used to help differentiate between MCTP control<br>Request messages and other message classes. Refer to 11.7.                                                                                  |
| D-bit         | Datagram bit. This bit is used to indicate whether the Instance ID field is<br>being used for tracking and matching requests and responses or is just<br>being used to identify a retransmitted message. Refer to 11.7. |

| Field Name                                    | Description                                                                                                                                                                                                                                                                                                                                                                                                                       |
|-----------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Instance ID                                   | The Instance ID field is used to identify new instances of an MCTP control<br>Request or Datagram to differentiate new requests or datagrams that are<br>sent to a given message terminus from retried messages that are sent to<br>the same message terminus. The Instance ID field is also used to match up<br>a particular instance of an MCTP Response message with the<br>corresponding instance of an MCTP Request message. |
| Command Code                                  | For Request messages, this field is a command code indicating the type of<br>MCTP operation the packet is requesting. Command code values are<br>defined in Table 12. The format and definition of request and response<br>parameters for the commands is given in Clause 12. The Command Code<br>that is sent in a Request shall be returned in the corresponding Response.                                                      |
| Completion Code                               | This field is only present in Response messages. This field contains a<br>value that indicates whether the response completed normally. If the<br>command did not complete normally, the value can provide additional<br>information regarding the error condition. The values for completion codes<br>are specified in Table 13.                                                                                                 |
| Message Data                                  | Zero or more bytes of parameter data that is specific to the particular<br>Command Code and whether the message is a Request or Datagram, or a<br>Response.                                                                                                                                                                                                                                                                       |
| * These fields are MCTP base protocol fields. |                                                                                                                                                                                                                                                                                                                                                                                                                                   |

## <span id="page-64-0"></span>1823 **11.6 MCTP control message transmission unit size**

- 1824 All MCTP control messages are required to have a packet payload that is no larger than the baseline 1825 transmission unit size of 64 bytes.
- 1826 MCTP control messages are carried in a single MCTP packet. Multiple messages are used if an operation 1827 requires more data to be transferred than can be carried in a single message.

## <span id="page-64-1"></span>1828 **11.7 Tag Owner (TO), Request (Rq), and Datagram (D) bit usage**

 For MCTP control messages, the Rq bit shall be set to 1b if the message is a "command" or Request message and 0b if the message is a Response message. For Datagram and Broadcast messages, the Rq bit shall always be set to 1b. MCTP Control messages that have unexpected or incorrect flag bit values shall be silently discarded by the receiver of the message.

- 1833 For the present specification, Requests and Datagrams are only issued from tag owners (TO bit = 1b). 1834 Provision has been left for the definition of possible future Datagrams that are not issued from tag owners
- 1835 (see [Table 11\)](#page-64-2).

| 1836 | Table 11 – Tag Owner (TO), Request (Rq) and Datagram (D) bit usage |
|------|--------------------------------------------------------------------|
|------|--------------------------------------------------------------------|

<span id="page-64-2"></span>

| MCTP Control Message Class                                             | Destination<br>EID Value | Tag<br>Owner<br>(TO) bit | Request<br>(Rq) bit | Datagram<br>(D) bit |
|------------------------------------------------------------------------|--------------------------|--------------------------|---------------------|---------------------|
| Command/Request                                                        | Target EID               | 1b                       | 1b                  | 0b                  |
| Responses are expected and tracked by<br>Instance ID at the requester. |                          |                          |                     |                     |
| Response                                                               | Target EID               | 0b                       | 0b                  | 0b                  |
| Broadcast Request                                                      | Broadcast EID            | 1b                       | 1b                  | 0b                  |
| Responses are expected and tracked by<br>Instance ID at the requester. |                          |                          |                     |                     |

| MCTP Control Message Class                                                                                                                                                               | Destination<br>EID Value | Tag<br>Owner<br>(TO) bit | Request<br>(Rq) bit | Datagram<br>(D) bit |
|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------|--------------------------|---------------------|---------------------|
| Datagram                                                                                                                                                                                 | Target EID               | 1b                       | 1b                  | 1b                  |
| Unacknowledged Request – Responses are<br>neither expected nor tracked by Instance ID at<br>the requester. Duplicate packets are handled the<br>same as retried Command/Request packets. |                          |                          |                     |                     |
| Broadcast Datagram (unacknowledged control<br>command that is broadcast.)                                                                                                                | Broadcast EID            | 1b                       | 1b                  | 1b                  |
| Reserved for future definition                                                                                                                                                           |                          | all other                |                     |                     |

## <span id="page-65-0"></span>1837 **11.8 Concurrent command processing**

## 1838 **11.8.1 Overview**

1839 This clause describes the specifications and requirements for handling concurrent overlapping MCTP 1840 control requests by endpoints.

## 1841 **11.8.2 Requirements for responders**

- 1842 An endpoint is not required to process more than one request at a time (that is, it can be "single threaded" 1843 and does not have to accept and act on new requests until it has finished responding to any previous 1844 request).
- 1845 A responder that is not ready to accept a new request can either silently discard the request, or it can 1846 respond with an ERROR\_NOT\_READY message completion code.
- 1847 A responder that can accept and process more than one request at a time is not required to return 1848 responses in the order that the requests were received.

## 1849 **11.8.3 Requirements for Requesters**

- 1850 An endpoint that issues MCTP control Requests to another endpoint shall wait until it gets the response 1851 to the particular request, or times out waiting for the response, before issuing a new request, Datagram, 1852 or Broadcast Datagram.
- 1853 An endpoint that issues MCTP control Requests is allowed to have multiple requests outstanding 1854 simultaneously to *different* responder endpoints.
- 1855 An endpoint that issues MCTP control Requests should be prepared to handle responses that may not 1856 match the request (that is, it should not automatically assume that a response that it receives is for a 1857 particular request). It should check to see that the command code and source EID values in the response 1858 match up with a corresponding outstanding command before acting on any parameters returned in the 1859 response.

## 1860 **11.8.4 Additional requirements for bridges**

- 1861 The packets that are routed *through* a bridge's routing functionality are not interpreted by the bridge and 1862 therefore are not considered to constitute concurrent requests.
- 1863 A bridge shall support at least one outstanding MCTP control request for each bus connection (port)
- 1864 through which MCTP control messages can be used to access the bridge's configuration and control 1865 functionality.

<span id="page-66-0"></span>1866 Bridges shall retain temporal ordering of packets forwarded from one message terminus to another.

## 1867 **12 MCTP control messages**

## <span id="page-66-1"></span>1868 **12.1 Overview**

1869 This clause contains detailed descriptions for each MCTP control message. The byte offsets for the

1870 Request and Response parameter information given in the tables for the commands indicates the byte

<span id="page-66-2"></span>1871 offset for the message data starting with the byte following the Command field.

## 1872 **12.2 MCTP control message command codes**

1873 [Table 12](#page-66-3) lists the MCTP control messages and their corresponding command code values. The

1874 commands and their associated parameters are specified later in this clause. For bridges, the

1875 requirements apply equally to all endpoints within the bridge device that are used to configure and control 1876 the bridges routing functionality.

| 1877 | Table 12 – MCTP control command numbers |
|------|-----------------------------------------|
|------|-----------------------------------------|

<span id="page-66-3"></span>

| Command |                                       |                                                                                                                                                           |            |            |        |  |
|---------|---------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|------------|------------|--------|--|
| Code    | Command Name                          | General Description                                                                                                                                       | E          | B          | Clause |  |
| 0x00    | Reserved                              | Reserved                                                                                                                                                  | –          | –          | –      |  |
| 0x01    | Set Endpoint ID                       | Assigns an EID to the endpoint at the given<br>physical address                                                                                           | Ma<br>Ng   | Ca1<br>Mg  | 12.4   |  |
| 0x02    | Get Endpoint ID                       | Returns the EID presently assigned to an<br>endpoint. Also returns information about what<br>type the endpoint is and its level of use of static<br>EIDs. | Ma<br>Og   | Ma<br>Og   | 12.5   |  |
| 0x03    | Get Endpoint UUID                     | Retrieves a per-device unique UUID associated<br>with the endpoint                                                                                        | Ca2<br>Og9 | Ca2<br>Og  | 12.6   |  |
| 0x04    | Get MCTP Version<br>Support           | Lists which versions of the MCTP control<br>protocol are supported on an endpoint                                                                         | Ma<br>Og   | Ma<br>Og5  | 12.7   |  |
| 0x05    | Get Message Type<br>Support           | Lists the message types that an endpoint<br>supports                                                                                                      | Ma<br>Og   | Ma<br>Og   | 12.8   |  |
| 0x06    | Get Vendor Defined<br>Message Support | Used to discover an MCTP endpoint's vendor<br>specific MCTP extensions and capabilities                                                                   | Oa<br>Og   | Oa<br>Og   | 12.9   |  |
| 0x07    | Resolve Endpoint ID                   | Used to get the physical address associated<br>with a given EID                                                                                           | Na<br>Og   | Ma<br>Og   | 12.10  |  |
| 0x08    | Allocate Endpoint<br>IDs              | Used by the bus owner to allocate a pool of<br>EIDs to an MCTP bridge                                                                                     | Na<br>Ng   | Ma6<br>Mg6 | 12.11  |  |
| 0x09    | Routing Information<br>Update         | Used by the bus owner to extend or update the<br>routing information that is maintained by an<br>MCTP bridge                                              | Oa8<br>Og8 | Ma4<br>Mg4 | 12.12  |  |
| 0x0A    | Get Routing Table<br>Entries          | Used to request an MCTP bridge to return data<br>corresponding to its present routing table entries                                                       | Na<br>Og   | Ma<br>Og   | 12.13  |  |
| 0x0B    | Prepare for Endpoint<br>Discovery     | Used to direct endpoints to clear their<br>"discovered" flags to enable them to respond to<br>the Endpoint Discovery command                              | Ca3<br>Ng  | Ca3<br>Cg3 | 12.14  |  |
| 0x0C    | Endpoint Discovery                    | Used to discover MCTP-capable devices on a<br>bus, provided that another discovery<br>mechanism is not defined for the particular<br>physical medium      | Ca3<br>Cg3 | Ca3<br>Cg3 | 12.15  |  |

| Command                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |                               |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | OMC       |            |        |
|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|------------|--------|
| Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Command Name                  | General Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | E         | B          | Clause |
| 0x0D                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Discovery Notify              | Used to notify the bus owner that an MCTP<br>device has become available on the bus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Na<br>Cg3 | Ca3<br>Cg3 | 12.16  |
| 0x0E                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Get Network ID                | Used to get the MCTP network ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Ca7       | Ca7        | 12.17  |
| 0x0F                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Query Hop                     | Used to discover what bridges, if any, are in the<br>path to a given target endpoint and what<br>transmission unit sizes the bridges will pass for<br>a given message type when routing to the target<br>endpoint                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Na<br>Og  | Ma<br>Og   | 12.18  |
| 0x10                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Resolve UUID                  | Used by endpoints to find another endpoint<br>matching an endpoint that uses a specific UUID.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Na<br>Og  | Oa<br>Og   | 12.19  |
| 0x11                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Query rate limit              | Used to discover the data rate limit settings of<br>the given target for incoming messages.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Oa<br>Og  | Oa<br>Og   | 12.20  |
| 0x12                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Request TX rate limit         | Used to request the allowed transmit data rate<br>limit for the given endpoint for outgoing<br>messages.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Oa<br>Og  | Oa<br>Og   | 12.21  |
| 0x13                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Update rate limit             | Used to update the receiving side on change to<br>the transmit data rate which was not requested<br>by the receiver                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Oa<br>Og  | Oa<br>Og   | 12.22  |
| 0x14                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Query Supported<br>Interfaces | Used to discover the existing device MCTP<br>interfaces.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Oa<br>Og  | Oa<br>Og   | 12.23  |
| 0xF0 –<br>0xFF                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Transport Specific            | This range of control command numbers is<br>reserved for definition by individual MCTP<br>Transport binding specifications. Transport<br>specific commands are intended to be used as<br>needed for setup and configuration of MCTP on<br>a given media. A particular transport specific<br>command number many have different<br>definitions depending on the binding<br>specification. Transport specific commands<br>shall only be addressed to endpoints on the<br>same medium. A bridge is allowed to block<br>transport specific commands from being bridged<br>to different media. The general format of<br>Transport specific messages is specified in<br>clause 12.18. | -         | -          | 12.24  |
| Key for OMC (optional / mandatory / conditional) column:<br>E<br>=<br>non-bridge, non-bus owner endpoint (simple endpoint)<br>B<br>=<br>bridge / bus-owner endpoint<br>Ma<br>=<br>mandatory (required) to accept. The request shall be accepted by the endpoint and a response generated per the<br>following command descriptions.<br>Mg<br>=<br>mandatory to generate. The endpoint shall generate this request as part of its responsibilities for MCTP operation.<br>Oa<br>=<br>optional to accept<br>Og<br>=<br>optional to generate<br>Ca<br>=<br>conditional to accept (see notes)<br>Cg<br>=<br>conditional to generate (see notes)<br>Na<br>=<br>not applicable to accept. This command is not applicable to the device type and shall not be accepted<br>Ng<br>=<br>not applicable to generate. This command is used for MCTP configuration and initialization and should not be<br>generated.<br>1.<br>The topmost bus owner is not required to support the Set Endpoint ID command.<br>2.<br>Hot-plug and add-in devices, and non-bridge devices that connect to multiple busses, are required to support the Get<br>Endpoint UUID command. See 8.18.8 and 8.18.9 for more info. |                               |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |           |            |        |
| 3.<br>Mandatory on a per-bus basis to support endpoint discovery if required by the physical transport binding used for the<br>particular bus type. Refer to the appropriate MCTP physical transport binding specification.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |                               |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |           |            |        |

|      | Command                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |              |                                                                                                                                                                                                                                                     | OMC |   |        |
|------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----|---|--------|
| Code |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Command Name | General Description                                                                                                                                                                                                                                 | E   | B | Clause |
| 4.   | The topmost bus owner is not required to accept this command. The command is required to be generated when<br>downstream bridges require dynamic routing information from bus owners that they are connected to. Some<br>implementations may be configured where all routing information has been statically configured into the bridge and no<br>dynamically provided information is required. In this case, it is not required to support the command while the endpoints are<br>configured in that manner. |              |                                                                                                                                                                                                                                                     |     |   |        |
| 5.   | Bridges should use this command to verify that they are initializing devices that are compatible with their MCTP control<br>protocol version.                                                                                                                                                                                                                                                                                                                                                                 |              |                                                                                                                                                                                                                                                     |     |   |        |
| 6.   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |              | The endpoint is required to accept this command if it indicated support for a dynamic EID pool. The command shall be<br>generated by the endpoint if the configuration requires the endpoint to support allocating EID pools to downstream bridges. |     |   |        |
|      | this command.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |              | 7. See Clause 9 for more information regarding MCTP Network IDs and 12.17 regarding the implementation requirements of                                                                                                                              |     |   |        |
| 8.   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |              | While it is optional for an endpoint to receive a routing information update, the MCTP Base specification does not specify a<br>bridge or bus owner function that sends such updates to particular endpoints.                                       |     |   |        |
|      |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |              |                                                                                                                                                                                                                                                     |     |   |        |

<span id="page-68-0"></span>9. While it is optional for an endpoint to support this command, support of this command is mandatory both to generate and to accept for devices supporting rate limiting.

## 1878 **12.3 MCTP control message completion codes**

 The command/result code field is used to return management operation results for response messages. If a SUCCESS completion code is returned then the specified response parameters (if any) shall also be returned in the response. If an error completion code (not SUCCESS) is returned by the responder, unless otherwise specified, the responder shall not return any additional parametric data and the requester shall ignore any additional parameter data provided in the response (if any). See [Table 13](#page-68-1) for the completion 1884 codes.

#### 1885 **Table 13 – MCTP control message completion codes**

<span id="page-68-1"></span>

| Value         | Name                  | Description                                                                                                                                                                                                                                      |
|---------------|-----------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0x00          | SUCCESS               | The Request was accepted and completed normally.                                                                                                                                                                                                 |
| 0x01          | ERROR                 | This is a generic failure message. (It should not be used<br>when a more specific result code applies.)                                                                                                                                          |
| 0x02          | ERROR_INVALID_DATA    | The packet payload contained invalid data or an illegal<br>parameter value.                                                                                                                                                                      |
| 0x03          | ERROR_INVALID_LENGTH  | The message length was invalid. (The Message body<br>was larger or smaller than expected for the particular<br>request.)                                                                                                                         |
| 0x04          | ERROR_NOT_READY       | The Receiver is in a transient state where it is not ready<br>to receive the corresponding message.                                                                                                                                              |
| 0x05          | ERROR_UNSUPPORTED_CMD | The command code field of the received message is<br>unspecified or not supported on this endpoint. This<br>completion code shall be returned for any unsupported<br>command values received in MCTP control Request<br>messages.                |
| 0x80–<br>0xFF | COMMAND_SPECIFIC      | This range of completion code values is reserved for<br>values that are specific to a particular MCTP control<br>message. The particular values (if any) and their<br>definition is provided in the specification for the<br>particular command. |
| All other     | Reserved              | Reserved                                                                                                                                                                                                                                         |

## <span id="page-69-0"></span>**12.4 Set Endpoint ID**

The Set Endpoint ID command assigns an EID to an endpoint and sets its Discovered Flag for the

endpoint on the physical bus from which this command was received. This command should only be

issued by a bus owner to assign an EID to an endpoint at a particular physical address. Since it is

 assumed the Endpoint does not already have an EID assigned to it, or because the EID is unknown, the destination EID in the message will typically be set to the special null destination EID value.

 The Set Endpoint ID command is also used to provide the Physical Address and EID of the Bus Owner to an Endpoint. An Endpoint that needs to communicate with the Bus Owner may capture the physical address and EID that was used to deliver the Set Endpoint ID message.

 Note: Endpoints that are not the Bus Owner should not issue the Set Endpoint ID command because it can cause the receiver of the message to capture incorrect information for the Bus Owner's address.

 An MCTP bridge may elect to have a single EID for its functionality, rather than using an EID for each port (bus connection) that is connected to a different bus owner. See [9.1.3](#page-41-1) for more information. In this case, the bridge will accept its EID assignment from the "first" bus to deliver the Set Endpoint ID request to the bridge.

 It is recognized that different internal processing delays within a bridge can cause the temporal ordering of requests to be switched if overlapping requests are received over more than one bus. Therefore, which request is accepted by an implementation is not necessarily tied to the request that is first received at the

bridge, but instead will be based on which request is the first to be processed by the bridge.

 If an EID has already been assigned and the Set Endpoint ID command is issued to set a new EID, the command shall return a SUCCESSFUL completion code, and the response shall use the same EID value that was used as the destination EID in the Set Endpoint ID command.

 If an EID has already been assigned and the Set Endpoint ID command is issued from a different bus without forcing an EID assignment, the command shall return a SUCCESSFUL completion code, but the response parameters shall return an EID assignment status of "EID rejected".

 The Set Endpoint ID command functions in the same manner regardless of whether the endpoint uses a static EID. The only difference is that if an endpoint has a static EID, it uses that EID as its initial "default" EID value. The endpoint does not treat this initial EID as if it were assigned to it by a different bus owner. That is, the endpoint shall accept the EID assignment from the first bus that the command is received from, and shall track that bus as the originating bus for the EID for subsequent instances of Set Endpoint ID command. See [8.18.3](#page-34-2) for more information. The request and response parameters are specified in [Table 14.](#page-69-1)

#### **Table 14 – Set Endpoint ID message**

<span id="page-69-1"></span>

|              | Byte | Description                                                                                                                                                                                                                                                                        |
|--------------|------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data | 1    | Operation<br>[7:2] – reserved<br>[1:0] – Operation:<br>Set EID.<br>00b<br>Submit an EID for assignment. The given EID will be accepted<br>conditional upon which bus the device received the EID from (see<br>preceding text). A device where the endpoint is only reached through |
|              |      | one bus shall always accept this operation (provided the EID value is<br>legal).                                                                                                                                                                                                   |
|              |      | Force EID.<br>01b<br>Force EID assignment. The given EID will be accepted regardless of                                                                                                                                                                                            |
|              |      | whether the EID was already assigned through another bus. Note that if<br>the endpoint is forcing, the EID assignment changes which bus is being                                                                                                                                   |

|               | Byte | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
|---------------|------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|               |      | tracked as the originator of the Set Endpoint ID command. A device<br>where the endpoint is only reached through one bus shall always accept<br>this operation (provided the EID value is legal), in which case the Set<br>EID and Force EID operations are equivalent.<br>Reset EID (optional).<br>10b<br>This option only applies to endpoints that support static EIDs. If static<br>EIDs are supported, the endpoint shall restore the EID to the statically<br>configured EID value. The EID value in byte 2 shall be ignored. An<br>ERROR_INVALID_DATA completion code shall be returned if this<br>operation is not supported.<br>Set Discovered Flag.<br>11b<br>Set Discovered flag to the "discovered" state only. Do not change<br>present EID setting. The EID value in byte 2 shall be ignored.<br>Note that Discovered flag is only used for some physical transport<br>bindings. An ERROR_INVALID_DATA completion code shall be returned<br>if this operation is selected and the particular transport binding does not<br>support a Discovered flag. |
|               | 2    | Endpoint ID.<br>0xFF, 0x00 = illegal.<br>Endpoints are not allowed to be assigned the broadcast or null EIDs. It is<br>recommended that the endpoint return an ERROR_INVALID_DATA<br>completion code if it receives either of these values.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Response data | 1    | Completion code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
|               | 2    | [7:6] – reserved<br>[5:4] – EID assignment status:<br>00b = EID assignment accepted.<br>01b = EID assignment rejected. EID has already been assigned by<br>another bus owner and assignment was not forced.<br>10b = reserved.<br>11b = reserved.<br>[3:2] – reserved.<br>[1:0] – Endpoint ID allocation status (see 12.11 for additional information):<br>00b = Device does not use an EID pool.<br>01b = Endpoint requires EID pool allocation.<br>10b = Endpoint uses an EID pool and has already received an<br>allocation for that pool.<br>11b = reserved                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
|               | 3    | EID Setting.<br>If the EID setting was accepted, this value will match the EID passed in the<br>request. Otherwise, this value returns the present EID setting.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
|               | 4    | EID Pool Size.<br>This is the size of the dynamic EID pool that the bridge can use to assign<br>EIDs or EID pools to other endpoints or bridges. It does not include the<br>count of any additional static EIDs that the bridge may maintain. See 8.18.3<br>for more information. Note that a bridge always returns its pool size<br>regardless of whether it has already received an allocation.<br>0x00 = no dynamic EID pool.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

## <span id="page-71-0"></span>1919 **12.5 Get Endpoint ID**

1920 The Get Endpoint ID command returns the EID for an endpoint. This command is typically issued only by 1921 a bus owner to retrieve the EID that was assigned to a particular physical address. Thus, the destination 1922 EID in the message will typically be set to the special Physical Addressing Only EID value. The request

1923 and response parameters are specified in [Table 15.](#page-71-1)

| 1924 | Table 15 – Get Endpoint ID message |
|------|------------------------------------|
|------|------------------------------------|

<span id="page-71-1"></span>

|               | Byte | Description                                                                                                                                                                                                                                                                    |
|---------------|------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data  | –    | –                                                                                                                                                                                                                                                                              |
| Response data | 1    | Completion Code.                                                                                                                                                                                                                                                               |
|               | 2    | Endpoint ID.                                                                                                                                                                                                                                                                   |
|               |      | 0x00 = EID not yet assigned.                                                                                                                                                                                                                                                   |
|               | 3    | Endpoint Type.                                                                                                                                                                                                                                                                 |
|               |      | [7:6] = reserved                                                                                                                                                                                                                                                               |
|               |      | [5:4] = Endpoint Type:                                                                                                                                                                                                                                                         |
|               |      | 00b = simple endpoint                                                                                                                                                                                                                                                          |
|               |      | 01b = bus owner/bridge                                                                                                                                                                                                                                                         |
|               |      | 10b = reserved                                                                                                                                                                                                                                                                 |
|               |      | 11b = reserved                                                                                                                                                                                                                                                                 |
|               |      | [3:2] = reserved                                                                                                                                                                                                                                                               |
|               |      | [1:0] = Endpoint ID Type:                                                                                                                                                                                                                                                      |
|               |      | 00b = dynamic EID.                                                                                                                                                                                                                                                             |
|               |      | The endpoint uses a dynamic EID only.                                                                                                                                                                                                                                          |
|               |      | 01b = static EID supported.                                                                                                                                                                                                                                                    |
|               |      | The endpoint was configured with a static EID. The EID returned by this<br>command reflects the present setting and may or may not match the<br>static EID value.                                                                                                              |
|               |      | The following two status return values are optional. If provided, they shall<br>be supported as a pair in place of the static EID support status return. It<br>is recommended that this be implemented if the Reset EID option in the<br>Set Endpoint ID command is supported. |
|               |      | 10b = static EID supported.                                                                                                                                                                                                                                                    |
|               |      | Present EID matches static EID.                                                                                                                                                                                                                                                |
|               |      | The endpoint has been configured with a static EID. The present value is<br>the same as the static value.                                                                                                                                                                      |
|               |      | 11b = static EID supported. Present EID does not match static EID.<br>Endpoint has been configured with a static EID. The present value is<br>different than the static value.                                                                                                 |
|               |      | See 8.18.3 for more information.                                                                                                                                                                                                                                               |
|               | 4    | Medium-Specific Information.                                                                                                                                                                                                                                                   |
|               |      | This byte can hold additional information about optional configuration of the<br>endpoint on the given medium, such as whether certain types of timing or<br>arbitration are supported. This should only be used to report static<br>information.                              |
|               |      | This byte shall be returned as 0x00 unless otherwise specified by the<br>transport binding.                                                                                                                                                                                    |

## <span id="page-72-0"></span>**12.6 Get Endpoint UUID**

 The Get Endpoint UUID command returns a universally unique identifier (UUID), also referred to as a globally unique ID (GUID), for the management controller or management device. The command can be used to correlate a device with one or more EIDs. The format of the ID follows the byte (octet) format specified in [RFC4122. RFC4122](#page-8-7) specifies four different versions of UUID formats and generation algorithms suitable for use for a device UUID in IPMI. These are version 1 (0001b) "time based", and three "name-based" versions: version 3 (0011b) "MD5 hash", version 4 (0100b) "Pseudo-random", and version 5 "SHA1 hash". The version 1 format is recommended. However, versions 3, 4, or 5 formats are also allowed. A device UUID should never change over the lifetime of the device. The request and response parameters are specified in [Table 16.](#page-72-1)

- See [8.18.8](#page-37-0) and [8.18.9](#page-38-2) for additional requirements on the use of the Get Endpoint UUID command.

| 1936 | Table 16 – Get Endpoint UUID message format |
|------|---------------------------------------------|
|------|---------------------------------------------|

<span id="page-72-1"></span>

|               | Byte | Description                                  |  |
|---------------|------|----------------------------------------------|--|
| Request data  | –    | –                                            |  |
| Response data | 1    | Completion Code                              |  |
|               | 2:17 | UUID bytes 1:16, respectively (see Table 17) |  |

 The individual fields within the UUID are stored most-significant byte (MSB) first per the convention described in [RFC4122.](#page-8-7) See [Table 17](#page-72-2) for an example format.

#### <span id="page-72-2"></span>**Table 17 – Example UUID format**

| Field                  | UUID Byte | MSB |
|------------------------|-----------|-----|
| time low               | 1         | MSB |
|                        | 2         |     |
|                        | 3         |     |
|                        | 4         |     |
| time mid               | 5         | MSB |
|                        | 6         |     |
| time high and version  | 7         | MSB |
|                        | 8         |     |
| clock seq and reserved | 9         | MSB |
|                        | 10        |     |
| node                   | 11        | MSB |
|                        | 12        |     |
|                        | 13        |     |
|                        | 14        |     |
|                        | 15        |     |
|                        | 16        |     |

## <span id="page-73-0"></span>1940 **12.7 Get MCTP version support**

## 1941 **12.7.1 Overview**

1942 This command can be used to retrieve the MCTP base specification versions that the endpoint supports, 1943 and also the message type specification versions supported for each message type. The format of the 1944 request and response parameters for this message is given in [Table 18.](#page-73-1)

1945 More than one version number can be returned for a given message type by the Get MCTP Version 1946 Support command. This enables the command to be used for reporting different levels of compatibility 1947 and backward compatibility with different specification versions. The individual specifications for the given

1948 message type define the requirements for which versions number values should be used for that 1949 message type. Those documents define which earlier version numbers, if any, shall also be listed.

1950 The command returns a completion code that indicates whether the message type number passed in the 1951 request is supported or not. This enables the command to also be used to query the endpoint for whether 1952 it supports a given message type.

1953 NOTE Version numbers are listed from oldest to newest. Versioning commands and version formats for vendor-

1954 defined message types, 0x7E and 0x7F, are vendor-specific and considered outside the scope of this specification.

|--|--|

## 1955 **Table 18 – Get MCTP version support message**

<span id="page-73-1"></span>

|               | Byte | Description                                                                                                                                                                                                                                                 |
|---------------|------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data  | 1    | Message Type Number                                                                                                                                                                                                                                         |
|               |      | The Message Type Number to retrieve version information for:                                                                                                                                                                                                |
|               |      | =<br>return MCTP base specification version information.<br>0xFF                                                                                                                                                                                            |
|               |      | =<br>unspecified. Support of this command for vendor-defined<br>0x7E, 0x7F<br>message types is vendor implementation-specific and<br>considered outside the scope of this specification.                                                                    |
|               |      | =<br>return MCTP control protocol message version<br>0x00<br>information.                                                                                                                                                                                   |
|               |      | =<br>return version of DSP0241<br>0x01                                                                                                                                                                                                                      |
|               |      | 0x02,0x03<br>=<br>return version of DSP0261                                                                                                                                                                                                                 |
|               |      | Other<br>=<br>return version information for a given message type. See<br>MCTP ID for message type numbers. When a Message<br>Type Number references a binding spec, the reported<br>version is of the binding spec and not of the associated<br>base spec. |
| Response data | 1    | Completion Code                                                                                                                                                                                                                                             |
|               |      | 0x80 = message type number not supported                                                                                                                                                                                                                    |
|               | 2    | Version Number Entry count                                                                                                                                                                                                                                  |
|               |      | One-based count of 32-bit version numbers being returned in this response.<br>Numerically lower version numbers are returned first.                                                                                                                         |
|               | 3:6  | Version Number entry 1: The following descriptions are informational. Refer<br>to DSP4004 for the normative definition of version numbering of DMTF<br>specifications.<br>[31:24]<br>= major version number. This field is used to identify a version of    |
|               |      | the specification that includes changes that make it incompatible<br>with one or more functions that were defined in versions of the<br>specification that have an older (smaller) major version number.                                                    |
|               |      | [23:16]<br>= minor version number. This field is used to identify functional<br>additions to the specification that are backward compatible with                                                                                                            |

| Byte  | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
|-------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|       | older (smaller) minor version numbers that share the same major<br>version number.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
|       | [15:8]<br>=<br>update version number. This field is used for editorial updates to<br>the specification that do not define new functionality nor change<br>existing functionality over the given major.minor release. This field<br>is informational and should be ignored when checking versions for<br>interoperability.                                                                                                                                                                                                                                                           |
|       | [7:0]<br>=<br>"alpha" byte. This value is used for pre-release (work-in-progress)<br>versions of the specification. Pre-release versions of the<br>specification are backward compatible with specification versions<br>that have an older (smaller) minor version numbers that share the<br>same major version number. However, since the alpha value<br>represents a version of the specification that is presently under<br>development, versions that share the same major and minor<br>version numbers, but have different 'alpha' versions may not be<br>fully interoperable. |
|       | The encoding of the version number and alpha fields is provided in 12.7.2.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| (7:X) | Version Number Entries 2 through N.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
|       | Additional 32-bit major/minor version numbers, if any.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
|       | This field is only included when there are 2 or more Version Number entries.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |

## <span id="page-74-0"></span>1956 **12.7.2 Version field encoding**

1957 The version field is comprised of four bytes referred to as the "major", "minor", "update", and "alpha" 1958 bytes. These bytes shall be encoded as follows:

1959 The "major", "minor", and "update" bytes are BCD-encoded, and each byte holds two BCD digits. The 1960 "alpha" byte holds an optional alphanumeric character extension that is encoded using one of the 1961 alphabetic characters [a-z, A-Z] from the US-ASCII [\(RFC20\)](#page-8-12) Character Set. The semantics of these fields

1962 follows that specified in [DSP4004.](#page-8-11)

- 1963 The value 0x00 in the alpha field means that the alpha field is not used. Software or utilities that display 1964 the version number should not display any characters for this field.
- 1965 The value 0xF in the most-significant nibble of a BCD-encoded value indicates that the most-significant 1966 nibble should be ignored and the overall field treated as a single-digit value. Software or utilities that 1967 display the number should only display a single digit and should not put in a leading "0" when displaying 1968 the number.
- 1969 A value of 0xFF in the "update" field indicates that the field to be ignored. Software or utilities that display
- 1970 the version number should not display any characters for the field. 0xFF is not allowed as a value for the 1971 "major" or "minor" fields.
- 1972 EXAMPLES:
- 1973 Version 1.1.0 → 0xF1F1F000
- 1974 Version 3.1 → 0xF3F1FF00
- 1975 Version 1.0a → 0xF1F0FF61
- 1976 Version 3.7.10a → 0xF3F71061
- 1977 Version 10.11.7 → 0x1011F700

### <span id="page-75-0"></span>**MCTP Base Specification DSP0236**

## **12.7.3 MCTP base specification version number**

- MCTP implementations that follow this particular specification shall return the following version information in the response to the Get MCTP Version Support message when the Message Type
- parameter in the request is set to 0xFF (return MCTP base specification version information).
- The Version Number Entry 1 field shall be used to indicate backward compatibility with Version 1.0 of the base specification as:
- 1.0 [Major version 1, minor version 0, any update version, no alpha)]
- This is reported using the encoding as: 0xF1F0FF00
- The Version Number Entry 2 field shall be used to indicate backward compatibility with Version 1.1 of the base specification as:
- 1.1 [Major version 1, minor version 1, any update version, no alpha)]
- This is reported using the encoding as: 0xF1F1FF00
- The Version Number Entry 3 field shall be used to indicate backward compatibility with Version 1.2 of the base specification as:
- 1.2 [Major version 1, minor version 2, any update version, no alpha)]
- This is reported using the encoding as: 0xF1F2FF00
- The version of the MCTP base specification for this specification shall be reported in Version Number Entry as:
- **1.3.2** [Major version 1, minor version 3, update version 3, no alpha)]
- <span id="page-75-1"></span>This is reported using the encoding as: 0xF1F3F300

## **12.7.4 MCTP control protocol version information**

- MCTP implementations that follow this particular specification shall return the following version information in the response to the Get MCTP Version Support message when the Message Type parameter in the request is set to 0x00 (return MCTP control protocol version information).
- The Version Number Entry 1 field shall be used to indicate backward compatibility with Version 1.0 of the base specification Control Protocol as:
- 1.0 [Major version 1, minor version 0, any update version, no alpha)]
- This is reported using the encoding as: 0xF1F0FF00
- The Version Number Entry 2 field shall be used to indicate backward compatibility with Version 1.1 of the base specification Control Protocol as:
- 1.1 [Major version 1, minor version 1, any update version, no alpha)]
- This is reported using the encoding as: 0xF1F1FF00
- The Version Number Entry 3 field shall be used to indicate backward compatibility with Version 1.2 of the base specification Control Protocol as:
- 1.2 [Major version 1, minor version 2, any update version, no alpha)]

- This is reported using the encoding as: 0xF1F2FF00The version of the MCTP base specification Control
- Protocol for this specification shall be reported in Version Number Entry 4 as:
- **1.3.2** [Major version 1, minor version 3, update version 3, no alpha)]
- This is reported using the encoding as: 0xF1F3F300

## <span id="page-76-0"></span>**12.8 Get Message Type Support**

 The Get Message Type Support command enables management controllers to discover the MCTP control protocol capabilities supported by other MCTP endpoints, and get a list of the MCTP message types that are supported by the endpoint. The request and response parameters for this message are listed in [Table 19.](#page-76-2)

The response to this command may be specific according to which bus the request was received over

- (that is, a device that supports a given message type may not support that message type equally across
- all buses that connect to the device).

<span id="page-76-2"></span>

|               | Byte  | Description                                                                                                                                       |
|---------------|-------|---------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data  | –     | –                                                                                                                                                 |
| Response data | 1     | Completion Code.                                                                                                                                  |
|               | 2     | MCTP Message Type Count. One-based.<br>Number of message types in addition to the MCTP control message type that<br>is supported by this endpoint |
|               | (3:N) | List of Message Type numbers. One byte per number. See Table 3 and<br>MCTP Message Types table in MCTP ID.                                        |

## **Table 19 – Get Message Type Support message**

## <span id="page-76-1"></span>**12.9 Get Vendor Defined Message Support**

## **12.9.1 Overview**

 The Get Vendor Defined Message Support operation enables management controllers to discover whether the endpoint supports vendor-defined messages, and, if so, the vendors or organizations that defined those messages. The format and definition of the request and response parameters for this message is given in [Table 20.](#page-77-1)

2034 **Table 20 – Get Vendor Defined Message Support message**

<span id="page-77-1"></span>

|               | Byte       | Description                                                                                                                                                                                                                                                                                                                                                                                                                                    |
|---------------|------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data  | 1          | Vendor ID Set Selector<br>Indicates the specific capability set requested. Indices start at 0x00 and<br>increase monotonically by 1. If the responding endpoint has one or more<br>capability sets with indices greater than the requested index, it increments<br>the requested index by 1 and returns the resulting value in the response<br>message. The requesting endpoint uses the returned value to request the<br>next capability set. |
| Response data | 1          | Completion Code                                                                                                                                                                                                                                                                                                                                                                                                                                |
|               | 2          | Vendor ID Set Selector<br>0xFF = no more capability sets.                                                                                                                                                                                                                                                                                                                                                                                      |
|               | Var        | Vendor ID<br>A structured field of variable length that identifies the vendor ID format<br>(presently PCI or IANA) and the ID of the vendor that defined the capability<br>set. The structure of this field is specified in Figure 21 – Structure of Vendor<br>ID field for Get Vendor Defined capabilities message.                                                                                                                           |
|               | 2<br>bytes | 16-bit numeric value or bit field, as specified by the vendor or organization<br>identified by the vendor ID. This value is typically used to identify a particular<br>command set type or major version under the given vendor ID.                                                                                                                                                                                                            |

## <span id="page-77-2"></span>2035 **12.9.2 Vendor ID formats**

2036 [Figure 21](#page-77-0) shows the general structure of Vendor ID fields used in this specification. The first byte of the 2037 field contains the Vendor ID Format, a numeric value that indicates the definition space and format of the 2038 ID. The remainder of the field holds the Vendor ID Data with content and format as specified in [Table 21.](#page-78-1)

2039 The MCTP management controller or management device can pick which format is best suited for the 2040 device. In general, if the device does not already have an existing vendor ID that matches one of the 2041 specified formats, it is recommended that the IANA enterprise number format be used.

2042

## <span id="page-77-0"></span>2043 **Figure 21 – Structure of Vendor ID field for Get Vendor Defined capabilities message**

2044 **Table 21 – Vendor ID formats**

<span id="page-78-1"></span>

| Vendor ID<br>Format Name  | Vendor ID<br>Format | Vendor ID<br>Data Length | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
|---------------------------|---------------------|--------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| PCI Vendor ID             | 0x00                | 2                        | 16-bit Unsigned Integer. The PCI 2.3 specifications<br>state the following about the PCI vendor ID: "This<br>field identifies the manufacturer of the device. Valid<br>vendor identifiers are allocated by the PCI SIG to<br>ensure uniqueness. 0xFFFF is an invalid value for<br>the Vendor ID." However, for MCTP this value may<br>be used for identifying aspects other than the<br>manufacturer of the device, such as its use in the<br>Vendor Defined – PCI message type, where it<br>identifies the vendor or organization that defined a<br>particular set of vendor-defined messages. Thus, in<br>some uses, the ID may or may not correspond to the<br>PCI ID for the manufacturer of the device. |
| IANA Enterprise<br>Number | 0x01                | 4                        | 32-bit Unsigned Integer. The IANA enterprise<br>number for the organization or vendor expressed as<br>a 32-bit unsigned binary number. For example, the<br>enterprise ID for DMTF is 412 (decimal) or<br>0x0000_019C expressed as a 32-bit hexadecimal<br>number. The enterprise number is assigned and<br>maintained by the Internet Assigned Numbers<br>Authority, www.iana.org, as a means of identifying a<br>particular vendor, company, or organization.                                                                                                                                                                                                                                               |

## <span id="page-78-0"></span>2045 **12.10 Resolve Endpoint ID**

 This command is sent to the bus owner to resolve an EID into the physical address that shall be used to deliver MCTP messages to the target endpoint. The command takes an EID as an input parameter in the request and returns the EID and the physical address for routing to that EID (if any) in the response. The response data will also indicate if no mapping was available.

 An endpoint knows the physical address of the bus owner by keeping track of which physical address was used when the endpoint received its EID assignment through the Set Endpoint ID command. The endpoint can send this command to the bus owner using the null destination EID value. This eliminates the need for the endpoint to also keep track of the EID of the bus owner. The request and response parameters are specified in [Table 22.](#page-78-2)

| 2055 | Table 22 – Resolve Endpoint ID message |
|------|----------------------------------------|
|      |                                        |

<span id="page-78-2"></span>

|               | Byte | Description                                                                                                                                                                                                                                                                                                                                                                                                                                 |  |
|---------------|------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--|
| Request data  | 1    | Target Endpoint ID<br>This is the EID that the bus owner is being asked to resolve.                                                                                                                                                                                                                                                                                                                                                         |  |
| Response data | 1    | Completion Code                                                                                                                                                                                                                                                                                                                                                                                                                             |  |
|               | 2    | Bridge Endpoint ID<br>This is the EID for the endpoint that is providing the bridging server (if any)<br>that is required to access the target endpoint.<br>If the EID being returned matches the same value as the target EID, it<br>indicates that there is no bridging function that is required to access the<br>target endpoint (that is, the target EID is local to the bus that the Resolve<br>Endpoint ID request was issued over). |  |
|               | 3:N  | Physical Address.                                                                                                                                                                                                                                                                                                                                                                                                                           |  |

| Byte | Description                                                                                                                                                                                                                                                               |
|------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      | The size of this field is dependent on the particular MCTP physical transport<br>binding used for the bus that this data is being provided for. The size and<br>format of this field is defined as part of the corresponding physical transport<br>binding specification. |

## <span id="page-79-0"></span>**12.11 Allocate Endpoint IDs**

 Bus owners are responsible for allocating pools of EIDs to MCTP bridges that are lower in the bus hierarchy. This is done using the Allocate Endpoint IDs command. The EID for the bridge itself is assigned separately and is *not* part of the pool given with this command.

 The bus owner will typically use this command as part of the EID assignment process for a bus. When a device has been assigned an EID using the Set Endpoint ID command, the response to that command indicates whether the endpoint supports an EID pool. If the device indicates that it supports an EID pool, the bus owner can then issue the Allocate Endpoint IDs command to supply the pool of EIDs to the device.

 NOTE: The Allocate Endpoint IDs command can also cause a bridge to rebuild its routing table. See [12.12.3](#page-82-2) for more information.

 When an EID or EID pool that was previously allocated becomes unused (for example, due to a hot-swap removal), the bus owner shall reclaim the endpoint's EID or EID pool allocation. See [8.18](#page-33-0) for additional details.

 Referring to [Figure 22,](#page-80-0) there is a potential race condition with handling EID allocation. In the scenario shown in this figure, it is possible that device X and device Z might both be assigning EIDs to device Y at the same time. This also means that, unless steps are taken, device Z could allocate endpoints to device

Y only to have this overwritten by a set of endpoints assigned by device X.

 To prevent this, the Allocate Endpoint IDs command is only accepted from the "first" bus that provides the EID pool to the device. If another bus owner attempts to deliver an EID pool through another bus, the

request will be rejected unless an intentional over-ride is done.

## <span id="page-80-0"></span>2078 **Figure 22 – EID Pools from multiple bus owners**

2079 The Allocate Endpoint IDs message fields are described in [Table 23.](#page-80-1)

#### 2080 **Table 23 – Allocate Endpoint IDs message**

<span id="page-80-1"></span>

|              | Byte | Description                                                                                                                                                                                                                                                                                                                       |
|--------------|------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data | 1    | Operation Flags:                                                                                                                                                                                                                                                                                                                  |
|              |      | [7:2] – reserved.                                                                                                                                                                                                                                                                                                                 |
|              |      | [1:0] – Operation:                                                                                                                                                                                                                                                                                                                |
|              |      | 00b = Allocate EIDs.                                                                                                                                                                                                                                                                                                              |
|              |      | Submit an EID pool allocation. Do not force allocation. This enables the<br>allocation to be rejected if the bridge has already received its EID pool<br>from another bus. (See additional information in the following clauses.)                                                                                                 |
|              |      | 01b = Force allocation.                                                                                                                                                                                                                                                                                                           |
|              |      | Force bridge to accept this EID pool regardless of whether it has already<br>received its EID pool from another bus. This shall also cause a bridge to<br>rebuild its routing tables. See 12.12.3 for more information.                                                                                                           |
|              |      | 10b = Get allocation information                                                                                                                                                                                                                                                                                                  |
|              |      | Return the response parameters without changing the present allocation.<br>This can be used to query information on the dynamic pool of EIDs<br>presently allocated to the Endpoint, if any. If this operation is selected,<br>the Number of Endpoint IDs and Starting Endpoint ID parameters in the<br>request shall be ignored. |
|              |      | 11b = Reserved                                                                                                                                                                                                                                                                                                                    |
|              | 2    | Number of Endpoint IDs (Allocated Pool Size)<br>Specifies the number of EIDs in the pool being made available to this<br>Endpoint                                                                                                                                                                                                 |

|               | Byte | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
|---------------|------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|               |      | Specifying a count of 0x00 shall be legal. If 0x00 is accepted or forced (and<br>the bridge lacks a static EID pool) no EIDs shall be available for distribution<br>by the particular bridge.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
|               | 3    | Starting Endpoint ID<br>Specifies the starting EID for the range of EIDs being allocated in the pool.<br>When multiple EIDs are provided, the IDs are sequential starting with this<br>value as the first EID in the range.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| Response data | 1    | Completion Code<br>An error completion code (ERROR_INVALID_DATA should be returned)<br>shall be returned if the number of EIDs being allocated (Number of Endpoint<br>IDs) exceeds the Dynamic Endpoint ID Pool size. (This error condition does<br>not apply to when the number of endpoint IDs passed in the request is<br>0x00).                                                                                                                                                                                                                                                                                                                                                                                  |
|               | 2    | [7:2] – reserved<br>[1:0] –<br>00b = Allocation was accepted. In the case that the bridge has a<br>completely static EID pool, the bridge should not track which bus has<br>sourced the command and shall accept the allocation if the Number of<br>Endpoint IDs (Allocated Pool Size) is 0x00.<br>01b = Allocation was rejected. The Allocate Endpoint IDs command is<br>accepted only from the "first" bus that provides the EID pool to the<br>device. If another bus owner attempts to deliver an EID pool through<br>another bus, the request will be rejected unless an intentional over-ride is<br>done. (The rationale for this behavior is explained in the text of this<br>clause.)<br>10b, 11b = reserved |
|               | 3    | Endpoint ID Pool Size (Dynamic)<br>This value is the size of the EID pool used by this endpoint. This is the size<br>of the dynamic EID pool that the bridge can use to assign EIDs or EID pools<br>to other endpoints or bridges. It does not include the count of any additional<br>static EIDs that the bridge may maintain. See 8.18.3 for more information.                                                                                                                                                                                                                                                                                                                                                     |
|               | 4    | First Endpoint ID<br>This field specifies the first EID assigned to the pool for this endpoint. The<br>value is 0x00 if there are no EIDs assigned to the pool.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

## <span id="page-81-0"></span>2081 **12.12 Routing Information Update**

## 2082 **12.12.1 General**

- 2083 The Routing Information Update message is used by a bus owner to give routing information to a bridge 2084 for the bus on which the message is being received.
- 2085 Because the physical address format is based on the bus over which the request is delivered, the bus 2086 owner shall use the medium-specific physical address format for the addresses sent using this command.
- 2087 An MCTP bridge may be sent more than one instance of this command to transfer the update information. 2088 An integral number of routing information update entries shall be provided in the command (that is,
- 2089 routing information update entries cannot be split across instances of the command).

## 2090 **12.12.2 Adding and replacing entries**

 The recipient of this command shall check to see whether the information in the request corresponds to the EID for an existing entry for the bus over which the command was received. If so, it shall replace that entry with the new information. If an entry for a given EID or EID range does not already exist, it shall create new entries for the given EIDs. In some cases this may require the bridge to split existing entries into multiple entries.

 NOTE: A bus owner is only allowed to update entries that correspond to its bus. For each routing table entry that was created or updated through the Routing Information Update message, the bridge shall keep track of which bus it received the Routing Information Update from. This is necessary so that when a Routing Information Update is received from a particular bus, the bridge only updates entries that correspond to entries that were originally given to it from that bus.

## <span id="page-82-2"></span>2101 **12.12.3 Rebuilding routing tables**

2102 A bridge that receives and accepts the Allocate Endpoint IDs command with the "Force Allocation" bit set 2103 (1b) shall clear out and rebuild its routing table information. The bridge shall issue commands to reassign 2104 EIDs and re-allocate EID pools to all downstream devices. The request and response parameters are

2105 specified in [Table 24,](#page-82-0) and format information is provided in [Table 25.](#page-82-1)

| 2106 | Table 24 – Routing Information Update message |
|------|-----------------------------------------------|
|------|-----------------------------------------------|

<span id="page-82-0"></span>

|               | Byte        | Description                                                                                     |  |
|---------------|-------------|-------------------------------------------------------------------------------------------------|--|
| Request data  | 1           | Count of update entries (1-based)                                                               |  |
|               | see<br>text | One or more update entries, based on the given count, as illustrated in<br>Table 25             |  |
| Response data | 1           | Completion Code<br>0x80 = Insufficient space to add requested entries to internal routing table |  |

## <span id="page-82-1"></span>2107 **Table 25 – Routing Information Update entry format**

| Byte | Description                                                                                                                                                                                  |  |  |  |
|------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--|--|--|
| 1    | [7:4] – reserved                                                                                                                                                                             |  |  |  |
|      | [3:0] – Entry Type:                                                                                                                                                                          |  |  |  |
|      | 00b = entry corresponds to a single endpoint that is not serving as an<br>MCTP bridge                                                                                                        |  |  |  |
|      | 01b = entry reflects an EID range for a bridge where the starting EID is<br>the EID of the bridge itself and additional EIDs in the range are routed by<br>the bridge                        |  |  |  |
|      | 10b = entry is for a single endpoint that is serving as an MCTP bridge                                                                                                                       |  |  |  |
|      | 11b = entry is an EID range for a bridge, but does not include the EID of<br>the bridge itself                                                                                               |  |  |  |
| 2    | [7:0] Size of EID Range. The count of EIDs in the range.                                                                                                                                     |  |  |  |
| 3    | First EID in EID Range.                                                                                                                                                                      |  |  |  |
|      | The EID Range is sequential (for example, if the size of the EID Range is 3<br>and the First EID value given in this parameter is 21, the Entry covers EIDs<br>21, 22, and 23).              |  |  |  |
| 4:N  | Physical Address. The size and format of this field is defined as part of the<br>corresponding physical transport binding specification for the bus that this<br>data is being provided for. |  |  |  |

## <span id="page-83-0"></span>2108 **12.13 Get Routing Table Entries**

2109 This command can be used to request an MCTP bridge or bus owner to return data corresponding to its

2110 present routing table entries. This data is used to enable troubleshooting the configuration of routing

2111 tables and to enable software to draw a logical picture of the MCTP network. More than one instance of

2112 this command will typically need to be issued to transfer the entire routing table content.

2113 An integral number of routing table entries shall be provided in the response to this command (that is,

2114 routing table entries cannot be split across instances of the command). The request and response

2115 parameters are specified in [Table 26,](#page-83-1) and format information is provided in [Table 27.](#page-83-2)

### 2116 **Table 26 – Get Routing Table Entries message**

<span id="page-83-1"></span>

|               | Byte | Description                                                                                                                                                                                             |  |
|---------------|------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--|
| Request data  | 1    | Entry Handle (0x00 to access first entries in table)                                                                                                                                                    |  |
| Response data | 1    | Completion Code                                                                                                                                                                                         |  |
|               | 2    | Next Entry Handle (Use this value to request the next set of entries, if any.)<br>If the routing table data exceeds what can be carried in a single MCTP<br>control response.<br>0xFF = No more entries |  |
|               | 3    | Number of routing table entries being returned in this response                                                                                                                                         |  |
|               | 4:N  | One or more routing table entries, formatted per Table 27. This field will be<br>absent if the number of routing table entries is 0x00.                                                                 |  |

## <span id="page-83-2"></span>2117 **Table 27 – Routing Table Entry format**

| Byte                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Description                                                                                                                                                                                  |  |  |  |
|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--|--|--|
| 1                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Size of EID range associated with this entry                                                                                                                                                 |  |  |  |
| 2                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Starting EID                                                                                                                                                                                 |  |  |  |
| 3                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Entry Type/Port Number                                                                                                                                                                       |  |  |  |
|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | [7:6] – Entry Type:                                                                                                                                                                          |  |  |  |
|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | 00b = entry corresponds to a single endpoint that does not operate as an MCTP bridge                                                                                                         |  |  |  |
|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | 01b = entry reflects an EID range for a bridge where the starting EID is the EID of the bridge<br>itself and additional EIDs in the range are routed by the bridge                           |  |  |  |
|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | 10b = entry is for a single endpoint that serves as an MCTP bridge                                                                                                                           |  |  |  |
|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | 11b = entry is an EID range for a bridge, but does not include the EID of the bridge itself                                                                                                  |  |  |  |
|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | [5] – Dynamic/Static Entry.                                                                                                                                                                  |  |  |  |
| Indicates whether the entry was dynamically created or statically configured. Note that statically<br>configured routing information shall not be merged with dynamic information when reporting<br>entry information using this command. While an implementation may internally organize its<br>data that way, dynamic and statically configured routing shall be reported as separate entries.<br>Dynamically created entries include entries that were generated from the Routing Information<br>Update command as well as entries that were created as a result of the bridge doing EID<br>assignment and EID pool allocation as a bus owner. |                                                                                                                                                                                              |  |  |  |
|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | 0b = Entry was dynamically created                                                                                                                                                           |  |  |  |
|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | 1b = Entry was statically configured                                                                                                                                                         |  |  |  |
|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | [4:0] – Port number                                                                                                                                                                          |  |  |  |
|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | This value is chosen by the bridge device vendor and is used to identify a particular bus<br>connection that the physical address for the entry is defined under. In some cases, this number |  |  |  |

| Byte | Description                                                                                                                                                                                                                                                                                          |
|------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      | may correspond to an internal "logical" bus that is not directly connected to an external physical<br>bus. Port numbers are required to be static.                                                                                                                                                   |
|      | It is recommended, but not required, that the ports (bus connections) on the bridge be<br>numbered sequentially starting from 0x00. This specification does not define any requirements<br>or recommendations on how port numbers are assigned to corresponding physical connections<br>on a device. |
| 4    | Physical Transport Binding Identifier, according to DSP0239.                                                                                                                                                                                                                                         |
| 5    | Physical Media Type Identifier, according to DSP0239. This value is used to indicate what format<br>the following physical address data is given in.                                                                                                                                                 |
| 6    | Physical Address Size. The size in bytes of the following Physical Address field<br>The size is defined as part of the corresponding physical transport binding specification identified<br>by the physical media type identifier.                                                                   |
| 7:N  | Physical Address.<br>The size and format of this field is defined as part of the corresponding physical transport binding<br>specification.                                                                                                                                                          |
|      | The information given in this field is given MSB first. Any unused bits should be set to 0b.                                                                                                                                                                                                         |

## <span id="page-84-0"></span>**12.14 Prepare for Endpoint Discovery**

The Endpoint Discovery message is used to determine if devices on a bus communicate MCTP (see

[Table 28\)](#page-85-2). Whether this message is required depends on the particular medium. Currently, this message may be required only by a particular transport binding, such as PCI Express (PCIe) VDM, because other

bindings such as SMBus/I2C may use other mechanisms for determining this information.

Each endpoint (except the bus owner) on the bus maintains an internal flag called the "Discovered" flag.

 The Prepare for Endpoint Discovery command is issued as a broadcast Request message on a given bus that causes each endpoint on the bus to set their respective Discovered flag to the "undiscovered" state. The flag is subsequently set to the "discovered" state when the Set Endpoint ID command is received by the endpoint.

- An endpoint also sets the flag to the "undiscovered" state at the following times:
- Whenever the physical address associated with the endpoint changes or is assigned
- Whenever an endpoint first appears on the bus and requires an EID assignment
- During operation if an endpoint enters a state that requires its EID to be reassigned
- For hot-plug endpoints: After exiting any temporary state where the hot-plug endpoint was unable to respond to MCTP control requests for more than TRECLAIM seconds (where TRECLAIM is specified in the physical transport binding specification for the medium used to access the endpoint). See [8.18.6](#page-36-0) for additional information.
- Endpoints that have their Discovered flag set to "undiscovered" should use physical addressing for any received MCTP control message filtering.
- Only endpoints that have their Discovered flag set to "undiscovered" will respond to the Endpoint Discovery message. Endpoints that have the flag set to "discovered" will not respond.
- The destination EID for the Prepare for Endpoint Discovery message is set to the Broadcast EID value
- (see [Table 2\)](#page-25-1) in the request message to indicate that this is a broadcast message. The response
- message sets the destination EID to be the ID of the source of the request message, which is typically the
- EID of the bus owner. The request and response parameters are specified in [Table 28.](#page-85-2)

## **MCTP Base Specification DSP0236**

- The Prepare for Endpoint Discovery message has no effect on existing EID assignments. That is,
- endpoints shall normally retain their EIDs until they are explicitly changed via the Set Endpoint ID
- command, and shall not clear them after getting a "Prepare for Endpoint Discovery" command. (Note that
- endpoints may lose their EIDs under other conditions such as power state changes, etc., as described
- elsewhere in this specification.)
- The Endpoint Discovery and Prepare for Endpoint Discovery commands may only be supported on
- particular transport bindings (e.g. MCTP over PCIe Vendor Defined Messaging). If the binding does not use this discovery approach (e.g. SMBus/I2C) the endpoint shall return an ERROR\_UNSUPPORTED\_CMD
- completion status for those commands.

### **Table 28 – Prepare for Endpoint Discovery message**

<span id="page-85-2"></span>

|               | Byte | Description     |
|---------------|------|-----------------|
| Request data  | –    | –               |
| Response data | 1    | Completion Code |

## <span id="page-85-0"></span>**12.15 Endpoint Discovery**

 This command is used to discover endpoints that have their Discovered flag set to "undiscovered". Only endpoints that have their Discovered flag set to "undiscovered" will respond to this message. Endpoints that have the flag set to "discovered" will not respond.

 This message is typically sent as a Broadcast Request message by the bus owner using the Broadcast EID as the destination EID, though for testing purposes endpoints shall also accept and handle this command as a non-broadcast Request. Additionally, the request may be sent as a datagram, depending on the transport binding requirements. The request and response (if any) parameters are specified in [Table 29.](#page-85-3)

## **Table 29 – Endpoint Discovery message**

<span id="page-85-3"></span>

|               | Byte | Description     |
|---------------|------|-----------------|
| Request data  | –    | –               |
| Response data | 1    | Completion Code |

## <span id="page-85-1"></span>**12.16 Discovery Notify**

 This message is available for use as a common message for enabling an endpoint to announce its presence to the bus owner. This will typically be used as part of the endpoint discovery process when an MCTP device is hot-plugged onto or becomes powered up on an MCTP bus.

Whether and how this message is used for endpoint discovery depends on the particular physical

- transport binding specification. For example, the SMBus/I2C transport binding does not use this message for an endpoint to announce itself because it takes advantage of mechanisms that are already defined for SMBus.
- This message should only be sent from endpoints to the bus owner for the bus that the endpoint is on so
- it can notify the bus owner that the endpoint has come online and may require an EID assignment or
- update. Additionally, the request may be sent as a datagram, depending on the transport binding
- requirements. The request and response (if any) parameters are specified in [Table 30.](#page-86-2)

## **Table 30 – Discovery Notify message**

<span id="page-86-2"></span>

|               | Byte | Description     |
|---------------|------|-----------------|
| Request data  | –    | –               |
| Response data | 1    | Completion Code |

## <span id="page-86-0"></span>**12.17 Get Network ID**

 The Get Network ID command returns a universally unique identifier (UUID), also referred to as a globally unique ID (GUID), for a given MCTP network. Typically this command is sent to the topmost MCTP bus- owner since the topmost bus-owner has this knowledge. A Network ID is required for add-in MCTP networks (For example, an MCTP Network on an add-in card or module). A Network ID is not required for a fixed (not add-in) MCTP network provided there is only one network in the system implementation. A Network ID is required for fixed MCTP networks when more than one fixed network exists in the system implementation and is simultaneously accessible by a common entity such as system software.

The format of the ID follows the byte (octet) format specified in [RFC4122. RFC4122](#page-8-7) specifies four

different versions of UUID formats and generation algorithms suitable for use for a device UUID in IPMI.

These are version 1 (0001b) "time based", and three "name-based" versions: version 3 (0011b) "MD5

 hash", version 4 (0100b) "Pseudo-random", and version 5 "SHA1 hash". The version 1 format is recommended. However, versions 3, 4, or 5 formats are also allowed. A device UUID should never

change over the lifetime of the device. The request and response parameters are specified in [Table 16.](#page-72-1)

**Table 31 – Get Network ID message format**

<span id="page-86-3"></span>

|               | Byte | Description                                        |
|---------------|------|----------------------------------------------------|
| Request data  | –    | –                                                  |
| Response data | 1    | Completion Code                                    |
|               | 2:17 | Network ID bytes 1:16, respectively (see Table 17) |

 The individual fields within the UUID are stored most-significant byte (MSB) first per the convention described in [RFC4122.](#page-8-7) See [Table 17](#page-72-2) for an example format.

## <span id="page-86-1"></span>**12.18 Query Hop**

 This command can be used to query a bridge to find out whether a given EID shall be accessed by going through that bridge, and if so, whether yet another bridge shall be passed through in the path to the endpoint, or if the endpoint is on a bus that is directly connected to the bridge.

 The command also returns the information about the transmission unit information that the bridge supports in routing to the given target endpoint from the bus that the request was received over. See [9.5](#page-54-0) for more information.

 NOTE The physical transport binding for MCTP may place additional requirements on the physical packet sizes that can be used to transfer MCTP packet payloads, such as requiring that physical packet sizes be in 32-byte or 64- byte increments, or particular power of 2 increments (for example, 128, 256, 512, and so on).

The request and response parameters are specified in [Table 32.](#page-87-1)

<span id="page-87-1"></span>

|               | Byte | Description                                                                                                                                                                                                                                                                                                                                                                                      |
|---------------|------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data  | 1    | Target Endpoint ID<br>0x00, 0xFF = reserved. (An ERROR_INVALID_DATA completion code shall<br>be returned.)                                                                                                                                                                                                                                                                                       |
|               | 2    | Message type for which transmission unit information is being requested.<br>Use the MCTP control message type number unless another message type<br>is of interest.                                                                                                                                                                                                                              |
| Response data | 1    | Completion Code<br>An ERROR_INVALID_DATA completion code shall be returned if the target<br>EID is not covered by any entry in the bridge's routing table.                                                                                                                                                                                                                                       |
|               | 2    | EID of the next bridge that is used to access the target endpoint, if any<br>Note: This response depends on which bus port the Query Hop request is<br>received over.<br>If this EID is 0x0:                                                                                                                                                                                                     |
|               |      | The EID is covered by the bridge's routing table, but the target EID does<br>not require access by going through this bridge from the port the request<br>was received over. This response will be returned if the target EID is<br>already local to the bus over which the request is being received. This<br>response is also returned when the target EID is an EID for the bridge<br>itself. |
|               |      | If this EID is non-zero and is different than the target EID passed in request:<br>The EID being provided is the EID of the "next bridge" in the path to the                                                                                                                                                                                                                                     |
|               |      | target EID.<br>If this EID is equal to the target EID passed in request:<br>The target EID is accessed by going through this bridge and no<br>additional bridges shall be gone through to reach the target.                                                                                                                                                                                      |
|               | 3    | Message Type. This value either returns the message type that was given in<br>the request, or it returns 0xFF to indicate that the information is applicable to<br>all message types that are supported by the bridge.                                                                                                                                                                           |
|               | 4:5  | Maximum supported incoming transmission unit size in increments of<br>16 bytes, starting from the baseline transmission unit size (0x0000 = 64<br>bytes, 0x0001 = 80 bytes, and so on).                                                                                                                                                                                                          |
|               | 5:6  | Maximum supported outgoing transmission unit size in increments of<br>16 bytes, starting from the baseline transmission unit (0x0000 = 64 bytes,<br>0x0001 = 80 bytes, and so on). The responder will return whether this<br>transmission unit size is supported for MCTP packets that it transmits for the<br>given message type.                                                               |

## <span id="page-87-0"></span>2206 **12.19 Resolve UUID**

2207 This command is used to get information about an endpoint based on its UUID. This command may be 2208 sent from any endpoint to the bus owner. This command takes a UUID as a parameter in the request and 2209 returns a list of EIDs and physical addresses that matches this UUID.

2210 A bus owner that supports this command shall keep in the routing table entries the UUID of each of the

2211 endpoints. The UUID values can be found using a "Get Endpoint UUID" command.

2212 An endpoint knows the physical address of the bus owner by keeping track of which physical address

2213 was used when the endpoint received its EID assignment through the Set Endpoint ID command. The

2214 endpoint can send this command to the bus owner using the null destination EID value. This eliminates

- 2215 the need for the endpoint to also keep track of the EID of the bus owner. The request and response
- 2216 parameters are specified in [Table 33.](#page-88-1)

| 2217 | Table 33 – Resolve UUID message |
|------|---------------------------------|
|------|---------------------------------|

<span id="page-88-1"></span>

|               | Byte | Description                                                                                                                                                               |
|---------------|------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data  | 1:16 | Requested UUID                                                                                                                                                            |
|               | 17   | Entry Handle (0x00 to access first entries in table)                                                                                                                      |
| Response data | 1    | Completion Code                                                                                                                                                           |
|               | 2    | Next Entry Handle (Use this value to request the next set of entries, if any.) If<br>the EID table data exceeds what can be carried in a single MCTP control<br>response. |
|               |      | 0xFF = No more entries                                                                                                                                                    |
|               | 3    | Number of EID entries being returned in this response.                                                                                                                    |
|               | 4:N  | One or more routing table entries, formatted per Table 34. This field will be<br>absent if the number of EID entries is 0x00.                                             |

## 2218 **Table 34 – Resolve UUID message entry format**

<span id="page-88-2"></span>

| Byte | Description                                                                                                                                                                  |
|------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0    | EID                                                                                                                                                                          |
| 1    | Physical Transport Binding Type Identifier, according to MCTP ID specification (DSP0239).                                                                                    |
| 2    | Physical Media Type Identifier, according to MCTP ID specification (DSP0239). This value is used to<br>indicate what format the following physical address data is given in. |
| 3    | Physical Address Size.                                                                                                                                                       |
| 4:N  | Physical Address.                                                                                                                                                            |

## <span id="page-88-0"></span>2219 **12.20 Query rate limit**

2220 This command can be used to query an EID for its transmit rate limiting capabilities and its receive data 2221 rate requirements.

2222 This command can be used by a message originator to determine the data rate that this EID accepts. The 2223 command can also be used to query the present settings for the EID's transmit data rate capabilities and

- 2224 present setting.
- 2225 The request and response parameters are specified in [Table 35.](#page-88-3)

## 2226 **Table 35 – Query rate limit message**

<span id="page-88-3"></span>

|               | Byte | Description                                                                                                                                                                             |
|---------------|------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data  | -    | -                                                                                                                                                                                       |
| Response data | 1    | Completion Code                                                                                                                                                                         |
|               | 2:5  | Receive information: receive buffer size in bytes.                                                                                                                                      |
|               | 6:9  | Receive Information: maximum receive data rate limit, in baseline<br>transmission unit packets/sec. A value of 0x0 indicates the receiver is not<br>requesting limiting of the traffic. |

| Byte  | Description                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|-------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|       | Note: Unless otherwise specified, it should be assumed that the limit has<br>been defined for communication between two EIDs with the receiver in its<br>most typical modes of operation. The value is not a guarantee. Factors such<br>as transient loading, and a typical device states may mean the receiver will<br>be temporarily unable to receive at the rate given in this response.                                                     |
| 10:13 | Transmit Rate limiter capabilities: Maximum supported rate limit, in baseline<br>transmission unit packets/sec. A value of 0x0 means the device cannot<br>throttle its traffic.                                                                                                                                                                                                                                                                  |
| 14:17 | Transmit Rate limiter capabilities: Minimum supported rate limit, in baseline<br>transmission unit packets/sec. A value of 0x0 means the device cannot<br>throttle its traffic.                                                                                                                                                                                                                                                                  |
| 18:20 | Transmit Rate limiter capabilities: Maximum supported burst size.                                                                                                                                                                                                                                                                                                                                                                                |
| 21:23 | Present Transmit Rate Limit Burst Setting: The maximal burst size allowed<br>to be sent from this EID at one time.                                                                                                                                                                                                                                                                                                                               |
| 24:27 | Present Setting: EID Maximal Transmit data rate limit, in baseline<br>transmission unit packets/sec. A value of 0x0 means the rate limiter is not<br>active (When Rate Limiting is inactive, the EID will be transmitting at the<br>maximum rate for its present state).                                                                                                                                                                         |
| 28    | Transmit Rate limiter capabilities:<br>[7:2] – Reserved<br>[1] – Transmit Rate limiting operation capability<br>0b – Transmit Rate limiting on this EID is applied to requested and non<br>requested messages together<br>1b – Transmit Rate limiting on this EID is applied only to non-requested<br>messages<br>[0] - Rate limiting Support on EID<br>0b – Transmit Rate limiting is not supported<br>1b – Transmit Rate limiting is supported |

## <span id="page-89-0"></span>2227 **12.21 Request TX rate limit**

2228 This command can be used to configure an EID for its maximal transmit rate limitations settings.

2229 This command shall be used by a data-receiving device to request to configure a transmitting EID for the 2230 maximal allowed data rate from the transmitting endpoint to that data-receiving EID.

- 2231 The request and response parameters are specified in [Table 36.](#page-89-1)

2232 **Table 36 – Request TX rate limit message**

<span id="page-89-1"></span>

|              | Byte | Description                                                                                                                                                                                                                                                                                                             |
|--------------|------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data | 1:3  | EID transmit maximal burst size in MCTP packets. This value defines the<br>maximum number of back-to-back consecutive packets that are allowed to<br>be sent from this endpoint, which the receiving EID supports. The term<br>'back-to-back' means the packets are transmitted with the minimum delay<br>between them. |
|              |      | This value shall be set to at least 1 packet to enable rate-limiting. A value of<br>0 in this field shall be used only to disable rate-limiting.                                                                                                                                                                        |
|              | 4:7  | EID Maximal Transmit data rate limit, in baseline transmission unit<br>packets/sec.                                                                                                                                                                                                                                     |

|               | Byte | Description                                                                                                                                                      |
|---------------|------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Response data | 1    | Completion Code<br>An ERROR_INVALID_DATA shall be returned if the rate limit requested is<br>not supported.                                                      |
|               | 2:5  | EID transmit burst size in MCTP packets. This value defines the presently<br>used maximum total burst size allowed to be sent from this endpoint at one<br>time. |
|               | 6:9  | EID transmit data rate limit, as presently used, in baseline transmission unit<br>packets/sec.                                                                   |

2233 The response values for EID transmit burst size in MCTP packets, and EID transmit data rate limit, may 2234 differ from the requested values. This can happen when multiple requests from multiple source EIDs 2235 received with different request values sharing the same rate limiter. See description in [10.1.6.](#page-61-1)

 The response to this command is sent when the new rate is in effect when a change is performed or immediately when no change is done. Following sending a response to [Request TX rate limit](#page-89-0) command for the first time from any EID, it is recommended that the endpoint receiving this command will send [Get](#page-72-0)  [Endpoint UUID](#page-72-0) command to the EID which sent the [Request TX rate limit](#page-89-0) command. This allows any device to identify when an endpoint is enumerated with a different EID, in order to properly calculate its rate-limiting settings.

## <span id="page-90-0"></span>2242 **12.22 Update rate limit**

2243 This command is sent from a transmitter EID to a receiver EID, to update a receiver on any change in the 2244 transmitter's rate settings, which did not originate from a request from the receiver. This command is sent 2245 to any connected receive EID which is not the EID which originated the rate change.

- 2246 The command shall be used only after a change of the EID transmit burst size and/or EID transmit data 2247 rate limit.
- 2248 The request and response parameters are specified in [Table 38.](#page-91-2)

#### 2249 **Table 37 – Update rate limit message**

<span id="page-90-2"></span>

|               | Byte | Description                                                                                                                                                      |
|---------------|------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data  | 1:3  | EID transmit burst size in MCTP packets. This value defines the presently<br>used maximum total burst size allowed to be sent from this endpoint at one<br>time. |
|               | 4:7  | EID transmit data rate limit, as presently used, in baseline transmission unit<br>packets/sec.                                                                   |
| Response data | 1    | Completion Code                                                                                                                                                  |

2250 If an error occurred on the transmitter which caused the rate limiting to be set to an unsupported rate, the 2251 receiver EID shall issue a new [Request TX rate limit](#page-89-0) command to the transmitter EID.

## <span id="page-90-1"></span>2252 **12.23 Query Supported Interfaces**

2253 This command can be used to query an endpoint for its MCTP interfaces capabilities.

2254 This command can be used by an MCTP device A to query the different interfaces which are available on 2255 MCTP device B for communicating MCTP messages between device A and B.

2256 The request and response parameters are specified in [Table 38.](#page-91-2)

## 2257 **Table 38 – Query Supported Interfaces**

<span id="page-91-2"></span>

|               | Byte | Description                                                                     |
|---------------|------|---------------------------------------------------------------------------------|
| Request data  | -    | -                                                                               |
| Response data | 1    | Completion Code                                                                 |
|               | 2    | Supported Interfaces Count (shall be ≥ 1)                                       |
|               | 3    | First interface Type (see MCTP physical medium identifiers table in<br>DSP0239) |
|               | 4    | First interface EID                                                             |
|               | …    |                                                                                 |
|               | …    |                                                                                 |
|               | N-1  | Last interface Type (see MCTP physical medium identifiers table in<br>DSP0239)  |
|               | N    | Last interface EID                                                              |

## <span id="page-91-0"></span>2258 **12.24 Transport Specific**

 Transport Specific commands are a range of commands that are available for use by transport binding specifications in order to perform additional MCTP Control functions that are defined by a particular transport binding. Transport specific commands shall only be addressed to endpoints on the same medium. A bridge is allowed to block transport specific commands from being bridged to different media.

2263 The request and response parameters are specified in [Table 39.](#page-91-3)

## 2264 **Table 39 – Transport Specific message**

<span id="page-91-3"></span>

|               | Byte | Description                                                                                                                                                                                                         |
|---------------|------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data  | 1    | MCTP Physical Transport Binding Identifier                                                                                                                                                                          |
|               |      | The ID of the Physical Transport specification that defines the transport<br>specific message. This ID is defined in MCTP physical medium identifiers<br>table in DSP0239 companion document to this specification. |
|               | 2    | MCTP Physical Media Identifier                                                                                                                                                                                      |
|               |      | The ID of the physical medium that the message is targeted for. This ID is<br>defined MCTP physical medium identifiers table in DSP0239 companion<br>document to this specification.                                |
|               | 3:N  | Transport specific command data. Defined by the transport binding<br>specification identified by the MCTP Physical Transport Binding Identifier<br>given in byte 1.                                                 |
|               |      | If the Physical Transport Binding Identifier = Vendor Defined:                                                                                                                                                      |
|               |      | The first four bytes of data shall be the IANA Enterprise ID for the<br>Vendor. MSB first. See 12.9.2 for the information on the IANA Enterprise<br>ID as used in this specification.                               |
| Response data | 1    | Completion Code                                                                                                                                                                                                     |

# <span id="page-91-1"></span>2265 **13 Vendor Defined – PCI and Vendor Defined – IANA messages**

2266 The Vendor Defined – PCI and Vendor Defined – IANA message types provide a mechanism for 2267 providing an MCTP message namespace for vendor-specific messages over MCTP.

- The PCI and IANA designations refer to the mechanism that is used to identify the vendor or organization
- this is specifying the message's functionality and any parametric data or other fields provided in the message body.
- Note that this specification only defines the initial bytes in the message body of these messages, and sets
- the requirement that these messages shall follow the requirements set by the MCTP base protocol and
- any additional requirements necessary to meet the transport of these messages over a particular medium, such as path transmission unit limitations.
- Otherwise, any other field definitions and higher-level message behavior such as retries, error/completion
- codes, and so on, is message type-specific and thus is vendor-specific.

## <span id="page-93-0"></span>2278 **13.1 Vendor Defined – PCI message format**

2279 For these messages, the MCTP message type is set to the value for "Vendor Defined – PCI" as defined in 2280 [Table 3.](#page-30-3) The request and response parameters are specified in [Table 40.](#page-93-2)

#### 2281 **Table 40 – Vendor Defined – PCI message format**

<span id="page-93-2"></span>

|                     | Byte    | Description                                                                                                                                           |
|---------------------|---------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data<br>1:2 |         | PCI/PCIe Vendor ID. Refer to PCIe. MSB first. This value is formatted per the<br>Vendor Data Field for the PCI Express vendor ID format. See 12.9.2". |
|                     |         | NOTE: Because the vendor ID format is implied by the command, the Vendor ID<br>Format bytes are not part of this field.                               |
|                     | (3:N+2) | Vendor-Defined Message Body. 0 to N bytes.                                                                                                            |
| Response data       | 1:2     | PCI/PCIe Vendor ID. Refer to PCIe. MSB first.                                                                                                         |
|                     | (3:M+2) | Vendor-Defined Message Body. 0 to M bytes.                                                                                                            |

## <span id="page-93-1"></span>2282 **13.2 Vendor Defined – IANA message format**

2283 For these messages, the MCTP message type is set to the value for "Vendor Defined – IANA" as defined 2284 in [Table 3.](#page-30-3) The request and response parameters are specified in [Table 41.](#page-93-3)

#### 2285 **Table 41 – Vendor Defined – IANA message format**

<span id="page-93-3"></span>

|                     | Byte    | Description                                                                                                                                          |
|---------------------|---------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| Request data<br>1:4 |         | IANA Enterprise ID for Vendor. MSB first. This value is formatted per the<br>Vendor Data Field for the IANA enterprise vendor ID format. See 12.9.2. |
|                     |         | NOTE: Because the vendor ID format is implied by the command, the Vendor ID<br>Format bytes are not part of this field.                              |
|                     | (5:N+4) | Vendor-Defined Message Body. 0 to N bytes.                                                                                                           |
| Response data       | 1:4     | IANA Enterprise ID for the Vendor. MSB first.                                                                                                        |
|                     | (5:M+4) | Vendor-Defined Message Body. 0 to M bytes.                                                                                                           |

2286

<span id="page-94-0"></span>

| 2287                 |     |           | ANNEX A                                                                                                                                                                                                                         |
|----------------------|-----|-----------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 2288                 |     |           | (informative)                                                                                                                                                                                                                   |
| 2289                 |     |           |                                                                                                                                                                                                                                 |
| 2290                 |     |           | Notation                                                                                                                                                                                                                        |
| 2291                 | A.1 | Notations |                                                                                                                                                                                                                                 |
| 2292                 |     |           | Examples of notations used in this document are as follows:                                                                                                                                                                     |
| 2293<br>2294<br>2295 | •   | 2:N       | In field descriptions, this will typically be used to represent a range of byte offsets<br>starting from byte two and continuing to and including byte N. The lowest offset is on<br>the left, and the highest is on the right. |
| 2296<br>2297         | •   | (6)       | Parentheses around a single number can be used in message field descriptions to<br>indicate a byte field that may be present or absent.                                                                                         |
| 2298<br>2299<br>2300 | •   | (3:6)     | Parentheses around a field consisting of a range of bytes indicates the entire range<br>may be present or absent. The lowest offset is on the left, and the highest is on the<br>right.                                         |
| 2301<br>2302<br>2303 | •   | PCIe      | Underlined, blue text is typically used to indicate a reference to a document or<br>specification called out in 2, "Normative References", or to items hyperlinked within<br>the document.                                      |
| 2304                 | •   | rsvd      | Abbreviation for Reserved. Case insensitive.                                                                                                                                                                                    |
| 2305<br>2306         | •   | [4]       | Square brackets around a number are typically used to indicate a bit offset. Bit offsets<br>are given as zero-based values (that is, the least significant bit [LSb] offset = 0).                                               |
| 2307<br>2308         | •   | [7:5]     | A range of bit offsets. The most-significant is on the left, and the least-significant is on<br>the right.                                                                                                                      |
| 2309<br>2310         | •   | 1b        | The lower case "b" following a number consisting of 0s and 1s is used to indicate the<br>number is being given in binary format.                                                                                                |
| 2311                 | •   | 0x12A     | A leading "0x" is used to indicate a number given in hexadecimal format.                                                                                                                                                        |
| 2312                 |     |           |                                                                                                                                                                                                                                 |

- <span id="page-95-0"></span>
- 2315

# 2313 **ANNEX B** 2314 **(informative)**

# 2316 **Change log**

| Version | Date       | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
|---------|------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 1.0.0   | 2009-05-21 |                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| 1.1.0   | 2010-02-19 | Updated the glossary and the overview section, including additions for<br>MCTP host interfaces and descriptions of MCTP networks. Added support<br>for MCTP network IDs and the Get Network ID command. Addressed<br>Mantis issue: 0000417.<br>Added text to Clause 1 (Scope) referencing DSP0238, DSP0239 per WG<br>ballot comments.                                                                                                                                      |
| 1.2.0   | 2013-01-10 | Added Resolve UUID command. Clarified use of Control Protocol Version<br>and versioning for OEM commands, Prepare for Endpoint Discovery<br>command, and the Allocate Endpoint IDs command. Clarified requirements<br>on MCTP Control message flags and TO bit use. Changed command<br>requirements to allow an Endpoint to optionally accept or generate Routing<br>Information Update commands. Corrected typographic and formatting<br>errors.                          |
| 1.2.1   | 2014-10-09 | Corrected misuse of reserved EIDs in figures. Changed document<br>organization to place bridging clauses in a new first level clause "MCTP<br>Bridging". Added clarifications and clause on "Endpoint ID Retention".<br>Added more cross references and clarifications to better identify<br>requirements associated with the Get Endpoint UUID command.                                                                                                                   |
| 1.3.0   | 2016-11-24 | Added Rate Limiting. Fixed formatting errors and typos.<br>Added Query Supported Interfaces command                                                                                                                                                                                                                                                                                                                                                                        |
| 1.3.1   | 2019-11-19 | Added acknowledgements list<br>Corrected error in sections 12.7.3 and 12.7.4<br>Corrected bits field error in Table 14, Table 15 and Table 23                                                                                                                                                                                                                                                                                                                              |
| 1.3.2   | 2024-01-30 | Clarified dropped packets description in 8.7<br>Corrected typos in Table 12 and Table 13<br>Clarified setting of Discovered Flag in 12.4<br>Added reference to the interface IDs in DSP0239 in 12.23 and 12.24<br>Editorial fixes.<br>ISO compliancy sections numbering edits and terms definitions edits.<br>Aligned terms between Figure 4 and Table 3<br>Removed redundant terms and definitions.<br>Updated Table 9 to clarify the EID addressing in control response. |
| 1.3.3   | 2024-03-25 | Added definition for physical layer term in 3.1.12<br>Updated 12.14 to use address filtering by an undiscovered endpoint<br>Editorial fixes.                                                                                                                                                                                                                                                                                                                               |

2317
