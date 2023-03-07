
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
  toAddress: string;
  subject: string;
  body: string;
};

type SimpleEmail = {
  id: string;
  fromAddress: string;
  toAddress: string;
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
  SimpleEmail,
  Email,
};
