type Email = {
  id: string;
  exerciseId: string;
  fromAddress: string;
  toEntity: string;
  to?: string;
  replyTo?: string;
  subject: string;
  cc?: string;
  bcc?: string;
  body: string;
  sentAt: string;
  status: EmailStatus;
};

type EmailForm = {
  toAddresses: string[];
  replyToAddresses?: string[];
  ccAddresses?: string[];
  bccAddresses?: string[];
  subject: string;
  body: string;
};

type EmailVariable = {
  name: string;
  description: string;
};

export enum EmailStatus {
  Delivered = 'delivered',
  BeingSent = 'being sent',
  Bounced = 'bounced',
}

export type {
  Email,
  EmailForm,
  EmailVariable,
};
