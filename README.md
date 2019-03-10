# backtracker

This is a simple backtracker that locates radiation sources (and their geometry, but not yet!) given radiation dosage samples.
It takes into account the spontaneous gamma-radaiation emmitted by radioactive decay of unstable material. The application is built for 3D space and backtracks radiation sources using the Boltzmann Transport Principle.

There are 3 main parts to this project:
- Source Simulator (lib)
- Sampler (lib + bin)
- Backtracker (bin)

### Source Simulator
Simulates radiation sources at given position using the Boltzmann Transport Principle and provides API to get radiation dosage samples at any point in the given space. 

### Sampler
Uses the simulator library to generate radiation samples at random points in the space. The samples are written to a csv file for further use.

### Backtracker
Uses an approach similar to Monte Carlo Simulation to backtrack and locate the sources of radiation in the space.It generates a Probability Distribution of the presence of source and writes it out to a csv file.
