type Email = {
  id: string;
  exerciseId: string;
  userId?: string;
  fromAddress: string;
  toAddresses: string[];
  replyToAddresses?: string[];
  ccAddresses?: string[];
  bccAddresses?: string[];
  subject: string;
  body: string;
  statusType: EmailStatusType;
  statusMessage?: string;
  createdAt: string;
};

type EmailForm = {
  toAddresses: string[];
  replyToAddresses?: string[];
  ccAddresses?: string[];
  bccAddresses?: string[];
  subject: string;
  template: string;
  body: string;
  userId?: string;
};

type EmailTemplate = {
  name: string;
  content: string;
};

type EmailVariable = {
  name: string;
  description: string;
};

export enum EmailStatusType {
  Pending = 'Pending',
  Sent = 'Sent',
  Failed = 'Failed',
}

export type {
  Email,
  EmailForm,
  EmailTemplate,
  EmailVariable,
};
