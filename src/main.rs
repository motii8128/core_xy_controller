use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    msg::common_interfaces::{geometry_msgs, std_msgs}, 
    pr_info
};

fn main()->Result<(), DynError>
{
    let ctx = Context::new()?;

    let node = ctx.create_node("core_xy_controller", None, Default::default())?;

    let subscriber = node.create_subscriber::<geometry_msgs::msg::Twist>("/cmd_vel", None)?;

    let left_publisher = node.create_publisher::<std_msgs::msg::Float32>("/left_motor_power", None)?;
    let right_publisher = node.create_publisher::<std_msgs::msg::Float32>("/right_motor_power", None)?;

    let logger = Logger::new(node.get_name());

    let mut selector = ctx.create_selector()?;

    pr_info!(logger, "Start {}", node.get_name());

    selector.add_subscriber(
        subscriber, 
        Box::new(move |msg| {
            let mut left_msg = std_msgs::msg::Float32::new().unwrap();
            let mut right_msg = std_msgs::msg::Float32::new().unwrap();

            left_msg.data = (msg.linear.x + msg.linear.y) as f32;
            right_msg.data = (msg.linear.x - msg.linear.y) as f32;

            let _ = left_publisher.send(&left_msg);
            let _ = right_publisher.send(&right_msg);
        }),
    );

    loop {
        selector.wait()?;
    }
}