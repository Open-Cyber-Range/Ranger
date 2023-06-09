
type NewExercise = {
  name: string;
  sdlSchema?: string;
};

type Exercise = {
  id: string;
  createdAt: string;
  updatedAt: string;
} & NewExercise;

type UpdateExercise = NewExercise;

type EmailForm = {
  toAddresses: string[];
  replyToAddresses?: string[];
  ccAddresses?: string[];
  bccAddresses?: string[];
  subject: string;
  body: string;
};

export enum EmailStatus {
  Delivered = 'delivered',
  BeingSent = 'being sent',
  Bounced = 'bounced',
}

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

export type {
  NewExercise,
  Exercise,
  UpdateExercise,
  EmailForm,
  Email,
};
