# GenM3

- version: `v0.1.0`
- author: [Will-YiFei Sheng](syf20020816@outlook.com)

```
<View> {
                        // style: {basic: { height: 160., width: 160.}}
                        height: 160.,
                        width: 160.,
                        show_bg: true,
                        draw_bg: {
                            color: #6750A4
                            instance loading: 1.0
                            
                            
                            fn sdf_box(p: vec2, b: vec2) -> float {
                                let d = abs(p) - b;
                                return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
                            }
                            
                            fn sdf_rounded_box(p: vec2, b: vec2, r: float) -> float {
                                let d = abs(p) - b + r;
                                return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0) - r;
                            }
                            
                            fn sdf_star(p: vec2, r: float, n: float, m: float) -> float {
                                let an = 3.141593 / n;
                                let en = 3.141593 / m;
                                let acs = vec2(cos(an), sin(an));
                                let ecs = vec2(cos(en), sin(en));
                                
                                let bn = mod(atan(p.x, p.y), 2.0 * an) - an;
                                let p_rot = length(p) * vec2(cos(bn), abs(sin(bn)));
                                let p_final = p_rot - r * acs;
                                let p_final2 = p_final - ecs * clamp(dot(p_final, ecs), 0.0, r * acs.y / ecs.y);
                                return length(p_final2) * sign(p_final.x);
                            }
                            
                            fn sdf_draw_star(p: vec2, r: float, teeth: float, th: float, tang: float) -> float {
                                let angle = atan(p.y, p.x);
                                let segment_angle = 2.0 * 3.141593 / teeth;
                                let local_angle = mod(angle + 3.141593, segment_angle) - segment_angle * 0.5;
                                let dist_from_center = length(p);
                                
                                // Create teeth pattern
                                let tooth_height = th;
                                let tooth_width = segment_angle * tang;
                                let tooth_factor = smoothstep_custom(tooth_width, 0.0, abs(local_angle));
                                let radius_variation = r + tooth_height * tooth_factor;
                                
                                return dist_from_center - radius_variation;
                            }

                            // Gear-like star shape for the first SVG
                            fn sdf_gear_star(p: vec2, r: float) -> float {
                                return sdf_draw_star(p, r, 10.0, 0.20, 0.58);
                            }
                            
                            // Smooth organic star for the second SVG  
                            fn sdf_organic_star(p: vec2, r: float) -> float {
                                return sdf_draw_star(p, r, 9.0, 0.2, 0.84);
                            }

                            // SDF helper functions
                            fn sdf_circle(p: vec2, r: float) -> float {
                                // return length(p) - r;
                                return sdf_draw_star(p, r, 2.0, 0.34, 0.86);
                            }

                            fn sdf_rounded(p: vec2, r: float) -> float {
                                return sdf_draw_star(p, r, 8.0, 0.25, 0.72);
                            }

                            fn sdf_hexagon(p: vec2, r: float) -> float {
                                return sdf_draw_star(p, r, 5.0, 0.12, 0.72);
                            }

                            fn sdf_octagon(p: vec2, r: float) -> float {
                                return sdf_draw_star(p, r, 4.0, 0.4, 0.76);
                            }

                            fn sdf_oval(p: vec2, r: vec2) -> float {
                                let k = vec2(1.0, r.y / r.x);
                                return (length(p * k) - r.x) / k.y;
                            }
                            
                            fn rotate(p: vec2, angle: float) -> vec2 {
                                let c = cos(angle);
                                let s = sin(angle);
                                return vec2(p.x * c - p.y * s, p.x * s + p.y * c);
                            }
                            
                            fn smoothstep_custom(edge0: float, edge1: float, x: float) -> float {
                                let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
                                return t * t * (3.0 - 2.0 * t);
                            }
                            
                            // Material 3 ease-in-out function for smooth transitions
                            fn ease_in_out(t: float) -> float {
                                if t < 0.5 {
                                    return 2.0 * t * t;
                                } else {
                                    return 1.0 - 2.0 * (1.0 - t) * (1.0 - t);
                                }
                            }
                            
                            // Enhanced easing for Material 3 style
                            fn material3_ease(t: float) -> float {
                                // Cubic bezier approximation for Material 3 standard easing
                                let t2 = t * t;
                                let t3 = t2 * t;
                                return 3.0 * t2 - 2.0 * t3;
                            }
                            
                            fn pixel(self) -> vec4 {
                                // Normalize coordinates to [-1, 1]
                                let uv = (self.pos * 2.0 - 1.0) * vec2(1.0, -1.0);
                                
                                // Material 3 style animation timing
                                // Shape transition cycle (every 14 seconds) for shape morphing
                                let shape_cycle_time = self.time * 0.14;    // Slow shape transitions
                                
                                let shape_index = mod(floor(shape_cycle_time), 7.0);
                                let raw_shape_progress = fract(shape_cycle_time);
                                
                                // Apply Material 3 easing to shape transitions
                                let eased_shape_progress = material3_ease(raw_shape_progress);
                                
                                // Rotation tied to shape progress - exactly one rotation per shape
                                // Each shape rotates exactly 360 degrees during its display time
                                let base_rotation = raw_shape_progress * 2.0 * 3.141593; // One full rotation (2π) per shape
                                
                                // Scale factor for shapes
                                let scale = 0.35;
                                let p = uv / scale;
                                
                                // Rotate the coordinate system
                                let rotated_p = rotate(p, base_rotation);
                                
                                // Calculate current and next shape distances
                                let mut current_dist = 1000.0;
                                let mut next_dist = 1000.0;
                                let next_shape_index = mod(shape_index + 1.0, 7.0);
                                
                                // Current shape
                                if shape_index < 0.5 { // Shape 1: Gear-like star (complex star)
                                    current_dist = sdf_gear_star(rotated_p, 0.8);
                                } else if shape_index < 1.5 { // Shape 2: Organic star (rounded star)
                                    current_dist = sdf_organic_star(rotated_p, 0.8);
                                } else if shape_index < 2.5 { // Shape 3: Hexagon
                                    current_dist = sdf_hexagon(rotated_p, 0.85);
                                } else if shape_index < 3.5 { // Shape 4: Circle
                                    current_dist = sdf_circle(rotated_p, 0.8);
                                } else if shape_index < 4.5 { // Shape 5: Rounded square/cross
                                    current_dist = sdf_rounded(rotated_p, 0.8);
                                } else if shape_index < 5.5 { // Shape 6: Octagon
                                    current_dist = sdf_octagon(rotated_p, 0.8);
                                } else { // Shape 7: Rounded square
                                    current_dist = sdf_oval(rotated_p, vec2(0.7, 0.4));
                                }
                                
                                // Next shape
                                if next_shape_index < 0.5 { // Shape 1: Gear-like star
                                    next_dist = sdf_gear_star(rotated_p, 0.8);
                                } else if next_shape_index < 1.5 { // Shape 2: Organic star
                                    next_dist = sdf_organic_star(rotated_p, 0.8);
                                } else if next_shape_index < 2.5 { // Shape 3: Hexagon
                                    next_dist = sdf_hexagon(rotated_p, 0.85);
                                } else if next_shape_index < 3.5 { // Shape 4: Circle
                                    next_dist = sdf_circle(rotated_p, 0.8);
                                } else if next_shape_index < 4.5 { // Shape 5: Rounded square/cross
                                    next_dist = sdf_rounded(rotated_p, 0.8);
                                } else if next_shape_index < 5.5 { // Shape 6: Octagon
                                    next_dist = sdf_octagon(rotated_p, 0.8);
                                } else { // Shape 7: Rounded square
                                    next_dist = sdf_oval(rotated_p, vec2(0.7, 0.4));
                                }
                                
                                // Material 3 style transition - starts late and ends early for longer hold times
                                let transition_start = 0.7;
                                let transition_end = 1.0;
                                let transition_progress = clamp((eased_shape_progress - transition_start) / (transition_end - transition_start), 0.0, 1.0);
                                let final_transition = ease_in_out(transition_progress);
                                
                                let final_distance = mix(current_dist, next_dist, final_transition);
                                
                                // Create smooth edges with anti-aliasing
                                let edge_smoothness = 0.015;
                                let alpha = 1.0 - smoothstep_custom(-edge_smoothness, edge_smoothness, final_distance);
                                
                                // Material 3 purple color with subtle pulsing
                                let base_color = self.color;
                                let pulse_speed = 1.5;
                                let color_variation = 0.92 + 0.08 * sin(self.time * pulse_speed);
                                let color = base_color * color_variation;
                                
                                // Add subtle glow effect for Material 3 style
                                let glow_intensity = 0.06;
                                let glow = exp(-abs(final_distance) * 5.0) * glow_intensity;
                                let final_alpha = min(alpha + glow, 1.0) * self.loading;
                                
                                return vec4(color.rgb, final_alpha);
                            }
                        }
                    }