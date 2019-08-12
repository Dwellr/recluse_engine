<p align="center">
    <img src="https://karrighan.github.io/recluse/img/git-banner.jpg" alt="recluse.io" />
</p>

<p align="center">
    <a href="https://discord.gg/vWbWVhq">
        <img src="https://img.shields.io/discord/601081333019574302.svg?logo=discord&colorB=7289DA">
    </a>
    <a href="https://liberapay.com/dotmal/donate">
        <img src="http://img.shields.io/liberapay/patrons/dotmal.svg?logo=liberapay">
    </a>
</p>

<p align="center">
    <strong>
        <a href="https://karrighan.github.io/recluse/docs">Documentation</a> | <a href="https://karrighan.github.io/recluse/feedback">Feedback</a> | <a href="https://karrighan.github.io/recluse/news">Announcements</a> | <a href="https://karrighan.github.io/recluse/social-media">Social Media</a>
    </strong>
</p>

<hr />

The Recluse Engine
==================
**Recluse** is a game engine. It uses [ncollide](http://ncollide.org/) for collision detection and [nalgebra](http://nalgebra.org/) for vector/matrix math and [ash](https://github.com/MaikKlein/ash)
for rendering and graphics.

<p align = "center">
    <strong>
        <em>Live examples coming soon!</em>
    </strong>
</p>

## Features
- Static, dynamic, and kinematic rigid bodies.
- Common convex primitives: box, ball, convex polyhedron.
- Concave geometries built from convex primitives: compound geometries, triangle mesh, polylines.
- Multibodies using reduced-coordinates approaches or constraints-based joints.
- Multibody joint limits and motors.
- Stable stacking due to non-linear a position-based penetration correction and one-shot contact manifold generation.
- Island based sleeping (objects deactivation when they are at rest).
- Ray casting.
- Swept sphere based continuous collision detection.
- Ball-in-socket joint.
- FixedJoint joint.
- Sensors.
- Deformable bodies (aka. soft-bodies)
- Kinematic bodies
- WASM support
- Rendering and more coming soon!

## Dependencies
The libraries needed to compile the engine are:

* [ncollide](http://ncollide.org): the collision detection library.
* [nalgebra](http://nalgebra.org): the linear algebra library.
* [ash](https://github.com/MaikKlein/ash): low-level Vulkan API bindings for Rust
