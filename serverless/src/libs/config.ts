export const config = {
  NODE_ENV: process.env.NODE_ENV,
  TERA_QUEUE_URL: process.env.QUEUE_URL,
  INFURA_KEY: "8328044ef20647ca8cf95216e364e9cb",
  ALCHEMY_KEY: "8uppuN2k88ZIrJleq7uVcQLqIuedvAO6",
  AWS_REGION: process.env.AWS_REGION || "us-west-2",
  DYNAMO_LOCAL_PORT: process.env.DYNAMO_LOCAL_PORT,
  ETH_L1_MESSAGE_TOPIC_ARN: process.env.ETH_L1_MESSAGE_TOPIC_ARN,
  ETH_L1_MESSAGE_TOPIC_NAME: process.env.ETH_L1_MESSAGE_TOPIC_NAME,
};
