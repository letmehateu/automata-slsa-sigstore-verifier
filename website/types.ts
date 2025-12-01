export interface NavItem {
  label: string;
  id: string;
}

export interface VerificationOutputField {
  id: number;
  label: string;
  description: string;
  technicalKey: string;
}

export interface ChatMessage {
  id: string;
  role: 'user' | 'model';
  text: string;
  timestamp: Date;
}

export enum SectionId {
  HERO = 'hero',
  PROTOCOL = 'protocol',
  BUNDLES = 'bundles',
  VERIFIER = 'verifier',
  CHAT = 'chat'
}
