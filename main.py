import pygame
import sys
import math

def draw_rectangle(x, y, width, height, color, rotation=0):
    """Draw a rectangle, centered at x, y.

    Arguments:
      x (int/float):
        The x coordinate of the center of the shape.
      y (int/float):
        The y coordinate of the center of the shape.
      width (int/float):
        The width of the rectangle.
      height (int/float):
        The height of the rectangle.
      color (str):
        Name of the fill color, in HTML format.
    """
    points = []

    # The distance from the center of the rectangle to
    # one of the corners is the same for each corner.
    radius = math.sqrt((height / 2)**2 + (width / 2)**2)

    # Get the angle to one of the corners with respect
    # to the x-axis.
    angle = math.atan2(height / 2, width / 2)

    # Transform that angle to reach each corner of the rectangle.
    angles = [angle, -angle + math.pi, angle + math.pi, -angle]

    # Convert rotation from degrees to radians.
    rot_radians = (math.pi / 180) * rotation

    # Calculate the coordinates of each point.
    for angle in angles:
        y_offset = -1 * radius * math.sin(angle + rot_radians)
        x_offset = radius * math.cos(angle + rot_radians)
        points.append((x + x_offset, y + y_offset))

    pygame.draw.polygon(screen, color, points)

    return points

def draw_pretty_line(surface, color, start_pos, end_pos, line_width):
    # Calculate the angle of the line
    angle = math.atan2(end_pos[1] - start_pos[1], end_pos[0] - start_pos[0])

    # Calculate the rotated endpoints
    rotated_start = (
        start_pos[0] - line_width / 2 * math.sin(angle),
        start_pos[1] + line_width / 2 * math.cos(angle),
    )
    rotated_end = (
        end_pos[0] - line_width / 2 * math.sin(angle),
        end_pos[1] + line_width / 2 * math.cos(angle),
    )

    # Create a polygon with rotated ends
    polygon_points = [
        rotated_start,
        rotated_end,
        (end_pos[0], end_pos[1]),
        (start_pos[0], start_pos[1]),
    ]

    # Draw the polygon on the surface
    pygame.draw.polygon(surface, color, polygon_points)

def distance_point_to_line(x0, y0, x1, y1, x2, y2):
    # Calculate the squared length of the line segment
    line_length_squared = (x2 - x1)**2 + (y2 - y1)**2
    
    # If the line segment is degenerate (zero length), return the distance to one of its endpoints
    if line_length_squared == 0:
        return math.sqrt((x0 - x1)**2 + (y0 - y1)**2)

    # Calculate the parametric position of the projection of the point onto the line segment
    t = max(0, min(1, ((x0 - x1) * (x2 - x1) + (y0 - y1) * (y2 - y1)) / line_length_squared))

    # Calculate the coordinates of the closest point on the line segment
    closest_x = x1 + t * (x2 - x1)
    closest_y = y1 + t * (y2 - y1)

    # Calculate the distance from the point to the closest point on the line segment
    distance = math.sqrt((x0 - closest_x)**2 + (y0 - closest_y)**2)

    return distance

def circle_inside_line(p, radius, l1, l2, width):
    distance = distance_point_to_line(p[0], p[1], l1[0], l1[1], l2[0], l2[1])
    return distance < (radius + width // 2)

def angle_speed_to_velocity(angle, speed):
    # Convert angle to radians
    angle_rad = math.radians(angle)

    # Calculate x and y components of the velocity vector
    velocity_x = speed * math.cos(angle_rad)
    velocity_y = speed * math.sin(angle_rad)

    return pygame.Vector2(velocity_x, velocity_y)


# Initialize Pygame
pygame.init()

# Constants
WIDTH, HEIGHT = 800, 600
FPS = 60

# Colors
WHITE = (255, 255, 255)

# Initialize the screen
screen = pygame.display.set_mode((WIDTH, HEIGHT), pygame.RESIZABLE)
pygame.display.set_caption("Linienfolger-Visualisierung")

# Clock to control the frame rate
clock = pygame.time.Clock()

# Car properties
car_width, car_height = 120, 80
car_x, car_y = 0, 100
car_speed = 1.5
car_angle = 0
car_updates_before_next_sensor_update = 5
car_sensor_pause_frames_left = car_updates_before_next_sensor_update

line_width = 18
points = [
(116, 68),
(317, 82),
(368, 29),
(454, 8),
(598, 35),
(653, 77),
(700, 158),
(694, 236),
(659, 275),
(562, 317),
(487, 320),
(497, 388),
(440, 386),
(383, 433),
(421, 514),
(368, 553),
(233, 527),
(220, 383),
(169, 311),
(58, 256),
(40, 170),
(59, 91),
(115, 68),
(116, 68),
(317, 82),
(368, 29),
(454, 8),
(598, 35),
(653, 77),
(700, 158),
(694, 236),
(659, 275),
(562, 317),
(487, 320),
(497, 388),
(440, 386),
(383, 433),
(421, 514),
(368, 553),
(233, 527),
(220, 383),
(169, 311),
(58, 256),
(40, 170),
(59, 91),
(115, 68),
(116, 68),
(317, 82),
(368, 29),
(454, 8),
(598, 35),
(653, 77),
(700, 158),
(694, 236),
(659, 275),
(562, 317),
(487, 320),
(497, 388),
(440, 386),
(383, 433),
(421, 514),
(368, 553),
(233, 527),
(220, 383),
(169, 311),
(58, 256),
(40, 170),
(59, 91),
(115, 68),
(116, 68),
(317, 82),
(368, 29),
(454, 8),
(598, 35),
(653, 77),
(700, 158),
(694, 236),
(659, 275),
(562, 317),
(487, 320),
(497, 388),
(440, 386),
(383, 433),
(421, 514),
(368, 553),
(233, 527),
(220, 383),
(169, 311),
(58, 256),
(40, 170),
(59, 91),
(115, 68),
]

# Sensors
sensor_radius = 18

last_left_mouse_state = False

time_off_line = 0.0

links = False
rechts = False

# Main game loop
while True:
    # Event handling
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            sys.exit()

    
    car_x += car_speed * pygame.math.Vector2(1, 0).rotate(-car_angle).x
    car_y += car_speed * pygame.math.Vector2(1, 0).rotate(-car_angle).y

    # Fill the screen with a white background
    screen.fill(WHITE)

    # Draw the rotated car
    car_points = draw_rectangle(car_x, car_y, car_width, car_height, 'yellow', car_angle)
    left_sensor = car_points[3]
    right_sensor = car_points[0]

    car_sensor_pause_frames_left -= 1
    if car_sensor_pause_frames_left == 0:
        links = False
        rechts = False
        for i in range(len(points) - 1):
            if circle_inside_line(left_sensor, sensor_radius, points[i], points[i+1], line_width):
                links = True
            if circle_inside_line(right_sensor, sensor_radius, points[i], points[i+1], line_width):
                rechts = True

        keys = pygame.key.get_pressed()
        car_sensor_pause_frames_left = car_updates_before_next_sensor_update

    if (links and not rechts) or (rechts and not links):
        time_off_line += 0.16
    else:
        time_off_line = 0.0

    time_off_line = min(time_off_line, 3)

    if links:
        car_angle -= time_off_line / 2.25
    if rechts:
        car_angle += time_off_line / 2.25

    car_velocity = angle_speed_to_velocity(car_angle, car_speed)


    if pygame.mouse.get_pressed()[0] and not last_left_mouse_state:
        points.append(pygame.mouse.get_pos())
    if pygame.key.get_pressed()[pygame.K_e]:
        for point in points:
            print(f"({point[0]}, {point[1]}),")

    last_left_mouse_state = pygame.mouse.get_pressed()[0]

    # Draw preview of line
    draw_pretty_line(screen, 'black', points[-1], pygame.mouse.get_pos(), line_width)

    # Draw sensor range
    if car_sensor_pause_frames_left == car_updates_before_next_sensor_update:
        pygame.draw.circle(screen, 'black', left_sensor, sensor_radius + 6)
        pygame.draw.circle(screen, 'black', right_sensor, sensor_radius + 6)

    pygame.draw.circle(screen, 'red' if links else 'green', left_sensor, sensor_radius)
    pygame.draw.circle(screen, 'red' if rechts else 'green', right_sensor, sensor_radius)

    # Draw lines
    for i in range(len(points)-1):
        draw_pretty_line(screen, 'black', points[i], points[i+1], line_width)

    # Update the display
    pygame.display.flip()

    # Cap the frame rate
    clock.tick(FPS)
