Référence :
* https://srenevey.github.io/ode-solvers/examples/kepler_orbit.html


<h1 id="kepler-orbit">Kepler Orbit</h1>
<h3 id="problem-definition">Problem Definition</h3>
<p>For this example, we consider a spacecraft on a Kepler orbit about the Earth and we want to predict its future trajectory. We assume that the elliptical orbit is described by the following classical orbital elements:</p>
<ul>
<li>Semi-major axis: a = 20,000 km</li>
<li>Eccentricity: e = 0.7</li>
<li>Inclination: i = 35°</li>
<li>Right ascension of the ascending node: &#937; = 100°</li>
<li>Argument of perigee: &#969; = 65°</li>
<li>True anomaly: &#957; = 30°</li>
</ul>
<p>The period of an orbit is given by <script type="math/tex">P = 2\pi\sqrt{\frac{a^3}{\mu}}</script> where  <script type="math/tex">\mu</script> is the standard gravitational parameter. For the Earth, &#956; = 398600.4354 km<sup>3</sup>/s<sup>2</sup>  and thus P = 2.8149 <script type="math/tex">\cdot</script> 10<sup>4</sup> s = 7.82 hours. The initial position of the spacecraft expressed in Cartesian coordinates is</p>
<p><center><script type="math/tex; mode=display">\mathbf{r} = \begin{bmatrix} -5007.2484 & -1444.9181 & 3628.5346 \end{bmatrix} \quad km</script></center></p>
<p>and velocity</p>
<p><center><script type="math/tex; mode=display">\mathbf{v} = \begin{bmatrix} 0.7177 & -10.2241 & 0.7482 \end{bmatrix} \quad km/s</script></center></p>
<p>such that the initial state vector is <script type="math/tex">\mathbf{s} = [\mathbf{r}, \mathbf{v}]^{T}</script>. The equations of motion describing the evolution of the system are</p>
<p><center><script type="math/tex; mode=display">\ddot{\mathbf{r}} = -\frac{\mu}{r^3}\mathbf{r}</script></center></p>
<p>where  <script type="math/tex">r</script> is the magnitude of <script type="math/tex">\mathbf{r}</script>. Since the crate handles first order ODEs, this system must be transformed into a state space form representation. An appropriate change of variables <script type="math/tex">y_1 = r_1</script>, <script type="math/tex">y_2 = r_2</script>, <script type="math/tex">y_3 = r_3</script>, <script type="math/tex">y_4 = \dot{r}_1 = v_1</script>, <script type="math/tex">y_5 = \dot{r}_2 = v_2</script>, <script type="math/tex">y_6 = \dot{r}_3 = v_3</script> yields</p>
<p>
<script type="math/tex; mode=display">\dot{y}_1 = y_4</script>
</p>
<p>
<script type="math/tex; mode=display">\dot{y}_2 = y_5</script>
</p>
<p>
<script type="math/tex; mode=display">\dot{y}_3 = y_6</script>
</p>
<p>
<script type="math/tex; mode=display">\dot{y}_4 = -\frac{\mu}{r^3}y_1</script>
</p>
<p>
<script type="math/tex; mode=display">\dot{y}_5 = -\frac{\mu}{r^3}y_2</script>
</p>
<p>
<script type="math/tex; mode=display">\dot{y}_6 = -\frac{\mu}{r^3}y_3</script>
</p>
<p>which can be numerically integrated.</p>