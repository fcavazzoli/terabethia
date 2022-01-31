import 'source-map-support/register';

import {
  formatJSONResponse,
  ValidatedEventAPIGatewayProxyEvent,
} from '@libs/apiGateway';
import { middyfy } from '@libs/lambda';
import {
  SQSClient,
  SendMessageCommand,
} from '@aws-sdk/client-sqs';
import schema from './schema';

const {
  QUEUE_URL,
} = process.env;

if (!QUEUE_URL) {
  throw new Error('QUEUE_URL must be set');
}

const sqsClient = new SQSClient({});

export const blockNativeEventHook: ValidatedEventAPIGatewayProxyEvent<
  typeof schema
> = async (event): Promise<any> => {
  if (!event.body) {
    return formatJSONResponse({
      statusCode: 500,
      body: 'Error blocknative hook: no data recieved!',
    });
  }

  try {
    await sqsClient.send(new SendMessageCommand({
      QueueUrl: QUEUE_URL,
      MessageBody: JSON.stringify(event.body),
      MessageDeduplicationId: event.body.hash,
    }));

    return formatJSONResponse({
      statusCode: 200,
      body: { message: 'success' },
    });
  } catch (error) {
    console.error('Exception on sns publish', error);

    return formatJSONResponse({
      statusCode: 500,
      body: 'Internal server error',
    });
  }
};

export const main = middyfy(blockNativeEventHook);