# trinity

multi-domain defensive control plane for ingress pressure, authentication abuse, and packet-level anomaly enforcement

designed for controlled enterprise environments and restricted infrastructure surfaces

no exposed learning layers  
no external dependency chains  
direct signal correlation with kernel-level enforcement  

---

## overview

trinity operates as a unified defensive system built on three independent signal domains:

* ingress pressure (connection saturation / volumetric behavior)
* authentication abuse (credential interaction anomalies)
* wire anomaly (packet-level irregularities and flow deviation)

each domain emits deterministic signals into a shared state layer consumed by an enforcement plane.

this design avoids delayed response models and eliminates reliance on external analysis pipelines.

---

## architecture

    trinity/

    ├── ingress/
    │   └── probe (xdp-based signal extraction)

    ├── auth/
    │   └── guard (authentication surface monitoring)

    ├── wire/
    │   └── sensor (packet inspection and anomaly detection)

    ├── core/
    │   └── enforcement (kernel-level decision layer)

    ├── maps/
    │   └── shared state definitions

    └── README.md

---

## execution model

ingress → pressure signal  
auth → abuse signal  
wire → anomaly signal  
core → enforcement  

all signals converge into shared maps and are evaluated in real time within the enforcement layer.

no asynchronous processing  
no queued decision paths  

---

## ingress domain

captures connection-level pressure using xdp hooks at interface ingress

behavioral focus:

* rapid connection bursts
* repeated source activity
* abnormal connection density per origin

implementation detail:

* eBPF maps used for per-source tracking
* atomic counters with bounded map sizes
* early-drop capability available at driver level

note:  
full parsing and key extraction logic intentionally minimized in public release

---

## authentication domain

monitors authentication surfaces for abuse patterns and credential interaction anomalies

behavioral focus:

* rapid retry sequences
* distributed credential attempts
* inconsistent request timing

controls:

* bounded request windows
* per-origin tracking
* immediate denial on threshold breach

note:  
rate correlation and adaptive controls are not exposed

---

## wire domain

performs packet-level inspection for anomaly detection outside normal flow characteristics

behavioral focus:

* packet rate irregularities
* protocol misuse patterns
* abnormal source behavior

implementation:

* passive inspection with minimal overhead
* selective state tracking
* anomaly signaling to shared state

note:  
deep inspection heuristics and pattern matching rules are restricted

---

## enforcement layer

kernel-level enforcement using xdp for immediate response

capabilities:

* drop decision at ingress
* zero-copy packet rejection
* constant-time lookup via maps

decision inputs:

* ingress pressure state
* authentication abuse state
* wire anomaly state

enforcement model:

* deterministic thresholds
* no probabilistic scoring
* no delayed mitigation

---

## shared state model

state is coordinated through bounded maps:

* per-source activity counters
* enforcement flags
* threshold indicators

design constraints:

* fixed-size allocation
* predictable lookup cost
* no dynamic resizing

---

## deployment model

requirements:

* linux kernel with eBPF/XDP support
* root-level access for interface attachment
* controlled network interface

execution flow:

1. compile ebpf components
2. attach enforcement program to interface
3. launch authentication monitor
4. activate packet sensor

all components operate independently but converge at enforcement layer

---

## security posture

* kernel-level mitigation (no user-space delay)
* strict signal isolation per domain
* deterministic behavior under load
* no external service reliance
* minimized attack surface

---

## operational constraints

* requires controlled deployment environment
* thresholds require manual tuning
* no adaptive learning included
* assumes trusted internal configuration

not intended for:

* public multi-tenant exposure
* unmanaged environments
* dynamic policy generation systems

---

## controlled exposure notice

this repository contains a reduced implementation surface.

the following components are intentionally limited:

* full packet parsing logic
* correlation heuristics
* adaptive thresholding
* enforcement tuning mechanisms

complete implementation remains restricted to controlled environments.

---

## extension vectors

* netlink-based control plane integration
* per-subnet aggregation layers
* protocol-aware anomaly classification
* integration with upstream filtering systems

---

## summary

trinity enforces a direct defensive pipeline:

observe → signal → correlate → enforce

no abstraction layers  
no deferred response  
no uncontrolled behavior  

designed for operators requiring deterministic control over ingress surfaces and authentication boundaries
