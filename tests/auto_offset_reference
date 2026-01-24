import math

def get_icon_center(vertices: list[(float, float)]) -> (float, float):
    avg_x = sum(vertex[0] for vertex in vertices) / len(vertices)
    avg_y = sum(vertex[1] for vertex in vertices) / len(vertices)
    icon_center = (avg_x, avg_y)
    return icon_center

def get_max_dist_icon_center_to_vertex(icon_center: (float, float), vertices: list[(float, float)]) -> float:
    max_dist_icon_center_to_vertex = 0
    for vertex in vertices:
        dist_icon_center_to_vertex = math.dist(icon_center, vertex)
        max_dist_icon_center_to_vertex = max(
            max_dist_icon_center_to_vertex,
            dist_icon_center_to_vertex
        )
    return max_dist_icon_center_to_vertex

def get_min_dist_icon_center_to_side_midpoint(icon_center: (float, float), vertices: list[(float, float)]) -> float:
    min_dist_icon_center_to_side_midpoint = float('inf')
    for vertex_index in range(len(vertices)):
        vertex = vertices[vertex_index]
        next_vertex = vertices[(vertex_index + 1) % len(vertices)]
        side_mid_point = (
            (vertex[0] + next_vertex[0]) / 2,
            (vertex[1] + next_vertex[1]) / 2
        )
        dist_icon_center_to_side_midpoint = math.dist(icon_center, side_mid_point)
        min_dist_icon_center_to_side_midpoint = min(
            min_dist_icon_center_to_side_midpoint,
            dist_icon_center_to_side_midpoint
        )
    return min_dist_icon_center_to_side_midpoint

def get_icon_radius(vertices: list[(float, float)]) -> (float, float):
    icon_center = get_icon_center(vertices)
    max_dist_icon_center_to_vertex = get_max_dist_icon_center_to_vertex(icon_center, vertices)
    min_dist_icon_center_to_side_midpoint = get_min_dist_icon_center_to_side_midpoint(icon_center, vertices)
    icon_radius = min(
        max_dist_icon_center_to_vertex,
        min_dist_icon_center_to_side_midpoint
    )
    return icon_radius

def get_shroud_center(vertices: list[(float, float)]) -> (float, float):
    icon_center = get_icon_center(vertices)
    icon_radius = get_icon_radius(vertices)
    print(icon_radius)
    shroud_center = (icon_center[0] - 0.5 * icon_radius, icon_center[1])
    return shroud_center

# Add you own shapes using the following as examples:
shapes = [
    [(-5, -5), (-5, 5), (5, 5), (5, -5)],   # Square
    [(0, 0), (0, 5), (5, 0)],               # Triangle
    [(-10, -10), (50, 100), (0, 0)],        # ???
    [(-10,-10), (-10,10), (10,10), (10,5), (15,5), (15,-5), (10,-5), (10,-10)]
]

for shape in shapes:
    print(f"{shape}\n\t{get_shroud_center(shape)}\n")