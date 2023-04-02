Référence :
* https://srenevey.github.io/ode-solvers/examples/kepler_orbit.html


Kepler Orbit
Problem Definition
For this example, we consider a spacecraft on a Kepler orbit about the Earth and we want to predict its future trajectory. We assume that the elliptical orbit is described by the following classical orbital elements:

Semi-major axis: a = 20,000 km
Eccentricity: e = 0.7
Inclination: i = 35°
Right ascension of the ascending node: Ω = 100°
Argument of perigee: ω = 65°
True anomaly: ν = 30°
The period of an orbit is given by P=2πa3μ−−√
 where μ
 is the standard gravitational parameter. For the Earth, μ = 398600.4354 km3/s2 and thus P = 2.8149 ⋅
 104 s = 7.82 hours. The initial position of the spacecraft expressed in Cartesian coordinates is

r=[−5007.2484−1444.91813628.5346]km
and velocity

v=[0.7177−10.22410.7482]km/s
such that the initial state vector is s=[r,v]T
. The equations of motion describing the evolution of the system are

r¨=−μr3r
where r
 is the magnitude of r
. Since the crate handles first order ODEs, this system must be transformed into a state space form representation. An appropriate change of variables y1=r1
, y2=r2
, y3=r3
, y4=r˙1=v1
, y5=r˙2=v2
, y6=r˙3=v3
 yields

y˙1=y4
y˙2=y5
y˙3=y6
y˙4=−μr3y1
y˙5=−μr3y2
y˙6=−μr3y3
which can be numerically integrated.