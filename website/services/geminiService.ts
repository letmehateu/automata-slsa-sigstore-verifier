import { GoogleGenAI, Chat, GenerateContentResponse } from "@google/genai";

const API_KEY = process.env.API_KEY || '';

let ai: GoogleGenAI | null = null;
let chatSession: Chat | null = null;

const SYSTEM_INSTRUCTION = `
You are an expert Software Security Engineer specializing in Software Supply Chain Security, specifically the Sigstore protocol and Zero-Knowledge Proofs (zkVMs).

Your role is to explain complex concepts to developers regarding:
1. The Sigstore Protocol (Fulcio, Rekor, OIDC, Cosign).
2. GitHub's attest-build-provenance workflow (Public vs Private bundles).
3. A specific Rust-based Verifier implementation that runs inside zkVMs (RiscZero, SP1, Brevis Pico).

Key technical details to know:
- The Rust verifier takes a Bundle JSON and a Trust Root JSON.
- It generates an on-chain verifiable proof.
- It outputs 11 specific fields including Fulcio Certificate Hashes, Subject Digest, OIDC details, and Timestamp info (RFC 3161 or Rekor).

Keep answers concise, technical but accessible, and focused on security benefits.
`;

export const initializeChat = (): void => {
  if (!API_KEY) {
    console.warn("Gemini API Key is missing.");
    return;
  }
  
  try {
    ai = new GoogleGenAI({ apiKey: API_KEY });
    chatSession = ai.chats.create({
      model: 'gemini-2.5-flash',
      config: {
        systemInstruction: SYSTEM_INSTRUCTION,
      },
    });
  } catch (error) {
    console.error("Failed to initialize Gemini:", error);
  }
};

export const sendMessageToGemini = async (message: string): Promise<string> => {
  if (!chatSession) {
    initializeChat();
    if (!chatSession) return "Error: AI Service not initialized (Missing API Key).";
  }

  try {
    const response: GenerateContentResponse = await chatSession!.sendMessage({
      message: message,
    });
    return response.text || "I couldn't generate a response.";
  } catch (error) {
    console.error("Error sending message to Gemini:", error);
    return "Sorry, I encountered an error processing your request.";
  }
};
